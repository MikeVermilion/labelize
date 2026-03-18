use std::io::Write;

use ab_glyph::{FontRef, PxScale};
use image::{Rgba, RgbaImage};
use imageproc::drawing;

use crate::barcodes;
use crate::elements::barcode_128::BarcodeMode;
use crate::elements::drawer_options::DrawerOptions;
use crate::elements::field_orientation::FieldOrientation;
use crate::elements::graphic_field::GraphicField;
use crate::elements::label_element::LabelElement;
use crate::elements::label_info::LabelInfo;
use crate::elements::label_position::LabelPosition;
use crate::elements::line_color::LineColor;
use crate::elements::text_field::TextField;
use crate::images;

use super::drawer_state::DrawerState;

static FONT_HELVETICA: &[u8] = crate::assets::FONT_HELVETICA_BOLD;
static FONT_DEJAVU_MONO: &[u8] = crate::assets::FONT_DEJAVU_SANS_MONO;
static FONT_DEJAVU_BOLD: &[u8] = crate::assets::FONT_DEJAVU_SANS_MONO_BOLD;
static FONT_GS: &[u8] = crate::assets::FONT_ZPL_GS;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Renderer
    }

    pub fn draw_label_as_png(
        &self,
        label: &LabelInfo,
        output: &mut dyn Write,
        options: DrawerOptions,
    ) -> Result<(), String> {
        let options = options.with_defaults();
        let mut state = DrawerState::new();

        let width_mm = options.label_width_mm;
        let height_mm = options.label_height_mm;
        let dpmm = options.dpmm;

        let label_width = (width_mm * dpmm as f64).ceil() as i32;
        let image_width = if label.print_width > 0 {
            label_width.min(label.print_width)
        } else {
            label_width
        };
        let image_height = (height_mm * dpmm as f64).ceil() as i32;

        let mut canvas = RgbaImage::from_pixel(
            image_width as u32,
            image_height as u32,
            Rgba([255, 255, 255, 255]),
        );

        let mut reverse_buf: Option<RgbaImage> = None;

        for element in &label.elements {
            let reverse_print = element.is_reverse_print();

            if reverse_print {
                let buf = reverse_buf.get_or_insert_with(|| {
                    RgbaImage::from_pixel(
                        image_width as u32,
                        image_height as u32,
                        Rgba([0, 0, 0, 0]),
                    )
                });
                // Clear buffer
                for pixel in buf.pixels_mut() {
                    *pixel = Rgba([0, 0, 0, 0]);
                }
                self.draw_element(buf, element, &options, &mut state)?;
                images::reverse_print::reverse_print(buf, &mut canvas);
            } else {
                self.draw_element(&mut canvas, element, &options, &mut state)?;
            }
        }

        // Handle print width centering and label inversion
        let invert_label = options.enable_inverted_labels && label.inverted;
        if image_width != label_width || invert_label {
            let mut final_canvas = RgbaImage::from_pixel(
                label_width as u32,
                image_height as u32,
                Rgba([255, 255, 255, 255]),
            );

            let offset_x = ((label_width - image_width) / 2) as i64;

            if invert_label {
                // Draw inverted (rotated 180)
                for y in 0..canvas.height() {
                    for x in 0..canvas.width() {
                        let src_pixel = *canvas.get_pixel(x, y);
                        let dst_x = (label_width as u32 - 1 - x).wrapping_add(offset_x as u32);
                        let dst_y = image_height as u32 - 1 - y;
                        if dst_x < final_canvas.width() && dst_y < final_canvas.height() {
                            final_canvas.put_pixel(dst_x, dst_y, src_pixel);
                        }
                    }
                }
            } else {
                image::imageops::overlay(&mut final_canvas, &canvas, offset_x, 0);
            }
            canvas = final_canvas;
        }

        let mut buf = Vec::new();
        images::monochrome::encode_png(&canvas, &mut buf)
            .map_err(|e| format!("failed to encode png: {}", e))?;
        output.write_all(&buf).map_err(|e| format!("failed to write png: {}", e))
    }

    fn draw_element(
        &self,
        canvas: &mut RgbaImage,
        element: &LabelElement,
        options: &DrawerOptions,
        state: &mut DrawerState,
    ) -> Result<(), String> {
        match element {
            LabelElement::Text(text) => self.draw_text(canvas, text, state),
            LabelElement::GraphicBox(gb) => {
                self.draw_graphic_box(canvas, gb);
                Ok(())
            }
            LabelElement::GraphicCircle(gc) => {
                self.draw_graphic_circle(canvas, gc);
                Ok(())
            }
            LabelElement::DiagonalLine(dl) => {
                self.draw_diagonal_line(canvas, dl);
                Ok(())
            }
            LabelElement::GraphicField(gf) => {
                self.draw_graphic_field(canvas, gf);
                Ok(())
            }
            LabelElement::Barcode128(bc) => self.draw_barcode_128(canvas, bc),
            LabelElement::BarcodeEan13(bc) => self.draw_barcode_ean13(canvas, bc),
            LabelElement::Barcode2of5(bc) => self.draw_barcode_2of5(canvas, bc),
            LabelElement::Barcode39(bc) => self.draw_barcode_39(canvas, bc),
            LabelElement::BarcodePdf417(bc) => self.draw_barcode_pdf417(canvas, bc),
            LabelElement::BarcodeAztec(bc) => self.draw_barcode_aztec(canvas, bc),
            LabelElement::BarcodeDatamatrix(bc) => self.draw_barcode_datamatrix(canvas, bc),
            LabelElement::BarcodeQr(bc) => self.draw_barcode_qr(canvas, bc, options),
            LabelElement::Maxicode(mc) => self.draw_maxicode(canvas, mc),
            _ => Ok(()), // Config/template elements are not drawn
        }
    }

    fn draw_text(
        &self,
        canvas: &mut RgbaImage,
        text: &TextField,
        state: &mut DrawerState,
    ) -> Result<(), String> {
        let font_data = get_ttf_font_data(&text.font.name);
        let font = FontRef::try_from_slice(font_data)
            .map_err(|e| format!("failed to load font: {}", e))?;

        let font_size = text.font.get_size() as f32;
        let scale_x = text.font.get_scale_x() as f32;
        let scale = PxScale { x: font_size * scale_x, y: font_size };

        // Measure text width approximately
        let text_width = measure_text_width(&text.text, &font, scale) as f64 * scale_x as f64;

        let (x, y) = get_text_top_left_pos(text, text_width, font_size as f64, state);
        state.update_automatic_text_position(text, text_width);

        let color = Rgba([0, 0, 0, 255]);

        // Handle text with block (word wrapping)
        if let Some(ref block) = text.block {
            let max_width = block.max_width as f32 / scale_x;
            let lines = word_wrap(&text.text, &font, scale, max_width);
            let line_height = font_size * (1.0 + block.line_spacing as f32 / font_size);

            let mut cy = y as f32;
            let max_lines = block.max_lines.max(1) as usize;
            for (i, line) in lines.iter().enumerate() {
                if i >= max_lines {
                    break;
                }
                let lx = match block.alignment {
                    crate::elements::text_alignment::TextAlignment::Center => {
                        let lw = measure_text_width(line, &font, scale) * scale_x;
                        x as f32 + (block.max_width as f32 - lw) / 2.0
                    }
                    crate::elements::text_alignment::TextAlignment::Right => {
                        let lw = measure_text_width(line, &font, scale) * scale_x;
                        x as f32 + block.max_width as f32 - lw
                    }
                    _ => x as f32,
                };
                drawing::draw_text_mut(canvas, color, lx as i32, cy as i32, scale, &font, line);
                cy += line_height;
            }
        } else {
            drawing::draw_text_mut(canvas, color, x as i32, y as i32, scale, &font, &text.text);
        }

        Ok(())
    }

    fn draw_graphic_box(
        &self,
        canvas: &mut RgbaImage,
        gb: &crate::elements::graphic_box::GraphicBox,
    ) {
        let color = line_color_to_rgba(gb.line_color);
        let x = gb.position.x;
        let y = gb.position.y;
        let w = gb.width.max(gb.border_thickness);
        let h = gb.height.max(gb.border_thickness);
        let border = gb.border_thickness;

        if gb.corner_rounding > 0 {
            // Draw filled rounded rectangle by drawing outer then clipping inner
            draw_filled_rect(canvas, x, y, w, h, color);
            if border < w.min(h) / 2 {
                let inner_color = Rgba([255, 255, 255, 255]);
                draw_filled_rect(canvas, x + border, y + border, w - 2 * border, h - 2 * border, inner_color);
            }
        } else {
            // Draw box with border
            if border >= w || border >= h {
                // Filled box
                draw_filled_rect(canvas, x, y, w, h, color);
            } else {
                // Top
                draw_filled_rect(canvas, x, y, w, border, color);
                // Bottom
                draw_filled_rect(canvas, x, y + h - border, w, border, color);
                // Left
                draw_filled_rect(canvas, x, y, border, h, color);
                // Right
                draw_filled_rect(canvas, x + w - border, y, border, h, color);
            }
        }
    }

    fn draw_graphic_circle(
        &self,
        canvas: &mut RgbaImage,
        gc: &crate::elements::graphic_circle::GraphicCircle,
    ) {
        let color = line_color_to_rgba(gc.line_color);
        let cx = gc.position.x as f32 + gc.circle_diameter as f32 / 2.0;
        let cy = gc.position.y as f32 + gc.circle_diameter as f32 / 2.0;
        let radius = gc.circle_diameter as f32 / 2.0;

        drawing::draw_hollow_circle_mut(
            canvas,
            (cx as i32, cy as i32),
            radius as i32,
            color,
        );
    }

    fn draw_diagonal_line(
        &self,
        canvas: &mut RgbaImage,
        dl: &crate::elements::graphic_diagonal_line::GraphicDiagonalLine,
    ) {
        let color = line_color_to_rgba(dl.line_color);
        let x = dl.position.x as f32;
        let y = dl.position.y as f32;
        let w = dl.width as f32;
        let h = dl.height as f32;

        if dl.top_to_bottom {
            drawing::draw_line_segment_mut(
                canvas,
                (x, y),
                (x + w, y + h),
                color,
            );
        } else {
            drawing::draw_line_segment_mut(
                canvas,
                (x, y + h),
                (x + w, y),
                color,
            );
        }
    }

    fn draw_graphic_field(&self, canvas: &mut RgbaImage, gf: &GraphicField) {
        let data_len = if gf.total_bytes > 0 {
            (gf.total_bytes as usize).min(gf.data.len())
        } else {
            gf.data.len()
        };

        if gf.row_bytes <= 0 || data_len == 0 {
            return;
        }

        let width = gf.row_bytes * 8;
        let height = data_len as i32 / gf.row_bytes;

        let mag_x = gf.magnification_x.max(1);
        let mag_y = gf.magnification_y.max(1);

        let black = Rgba([0, 0, 0, 255]);

        for y in 0..height {
            for x in 0..width {
                let idx = (y * (width / 8) + x / 8) as usize;
                if idx >= gf.data.len() {
                    continue;
                }
                let val = (gf.data[idx] >> (7 - x % 8)) & 1;
                if val != 0 {
                    for my in 0..mag_y {
                        for mx in 0..mag_x {
                            let px = (gf.position.x + x * mag_x + mx) as u32;
                            let py = (gf.position.y + y * mag_y + my) as u32;
                            if px < canvas.width() && py < canvas.height() {
                                canvas.put_pixel(px, py, black);
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw_barcode_128(
        &self,
        canvas: &mut RgbaImage,
        bc: &crate::elements::barcode_128::Barcode128WithData,
    ) -> Result<(), String> {
        let content = &bc.data;
        let img = match bc.barcode.mode {
            BarcodeMode::No => {
                let (img, _text) = barcodes::code128::encode_no_mode(content, bc.barcode.height, bc.width)?;
                img
            }
            _ => {
                barcodes::code128::encode_auto(content, bc.barcode.height, bc.width)?
            }
        };

        let pos = adjust_image_typeset_position(&img, &bc.position, bc.barcode.orientation);
        overlay_with_rotation(canvas, &img, &pos, bc.barcode.orientation);
        Ok(())
    }

    fn draw_barcode_ean13(
        &self,
        canvas: &mut RgbaImage,
        bc: &crate::elements::barcode_ean13::BarcodeEan13WithData,
    ) -> Result<(), String> {
        let img = barcodes::ean13::encode(&bc.data, bc.barcode.height, bc.width)?;
        let pos = adjust_image_typeset_position(&img, &bc.position, bc.barcode.orientation);
        overlay_with_rotation(canvas, &img, &pos, bc.barcode.orientation);
        Ok(())
    }

    fn draw_barcode_2of5(
        &self,
        canvas: &mut RgbaImage,
        bc: &crate::elements::barcode_2of5::Barcode2of5WithData,
    ) -> Result<(), String> {
        let content: String = bc.data.chars().filter(|c| c.is_ascii_digit()).collect();
        let img = barcodes::twooffive::encode(
            &content,
            bc.barcode.height,
            bc.width_ratio as i32,
            bc.width,
            bc.barcode.check_digit,
        )?;
        let pos = adjust_image_typeset_position(&img, &bc.position, bc.barcode.orientation);
        overlay_with_rotation(canvas, &img, &pos, bc.barcode.orientation);
        Ok(())
    }

    fn draw_barcode_39(
        &self,
        canvas: &mut RgbaImage,
        bc: &crate::elements::barcode_39::Barcode39WithData,
    ) -> Result<(), String> {
        let img = barcodes::code39::encode(
            &bc.data,
            bc.barcode.height,
            bc.width_ratio as i32,
            bc.width,
        )?;
        let pos = adjust_image_typeset_position(&img, &bc.position, bc.barcode.orientation);
        overlay_with_rotation(canvas, &img, &pos, bc.barcode.orientation);
        Ok(())
    }

    fn draw_barcode_pdf417(
        &self,
        canvas: &mut RgbaImage,
        bc: &crate::elements::barcode_pdf417::BarcodePdf417WithData,
    ) -> Result<(), String> {
        let img = barcodes::pdf417::encode(
            &bc.data,
            bc.barcode.row_height,
            bc.barcode.columns,
            bc.barcode.rows,
            bc.barcode.truncate,
        )?;
        let pos = adjust_image_typeset_position(&img, &bc.position, bc.barcode.orientation);
        overlay_with_rotation(canvas, &img, &pos, bc.barcode.orientation);
        Ok(())
    }

    fn draw_barcode_aztec(
        &self,
        canvas: &mut RgbaImage,
        bc: &crate::elements::barcode_aztec::BarcodeAztecWithData,
    ) -> Result<(), String> {
        let mag = bc.barcode.magnification.max(1);
        let img = barcodes::aztec::encode(&bc.data, mag, bc.barcode.size)?;
        let pos = adjust_image_typeset_position(&img, &bc.position, bc.barcode.orientation);
        overlay_with_rotation(canvas, &img, &pos, bc.barcode.orientation);
        Ok(())
    }

    fn draw_barcode_datamatrix(
        &self,
        canvas: &mut RgbaImage,
        bc: &crate::elements::barcode_datamatrix::BarcodeDatamatrixWithData,
    ) -> Result<(), String> {
        let scale = bc.barcode.height.max(1);
        let img_raw = barcodes::datamatrix::encode(&bc.data, scale)?;
        let pos = adjust_image_typeset_position(&img_raw, &bc.position, bc.barcode.orientation);
        overlay_with_rotation(canvas, &img_raw, &pos, bc.barcode.orientation);
        Ok(())
    }

    fn draw_barcode_qr(
        &self,
        canvas: &mut RgbaImage,
        bc: &crate::elements::barcode_qr::BarcodeQrWithData,
        _options: &DrawerOptions,
    ) -> Result<(), String> {
        let (input_data, _ec, _) = bc.get_input_data()?;
        let img = barcodes::qrcode::encode(&input_data, bc.barcode.magnification)?;

        let mut pos = bc.position.clone();
        if !pos.calculate_from_bottom {
            pos.y += bc.height;
        } else {
            let ft_offset = bc.barcode.magnification * 7;
            pos.y = (pos.y - img.height() as i32).max(0) - ft_offset;
        }

        overlay_at(canvas, &img, pos.x, pos.y);
        Ok(())
    }

    fn draw_maxicode(
        &self,
        canvas: &mut RgbaImage,
        mc: &crate::elements::maxicode::MaxicodeWithData,
    ) -> Result<(), String> {
        let _input_data = mc.get_input_data()?;
        let img = barcodes::maxicode::encode(&mc.data)?;
        let pos = adjust_image_typeset_position(&img, &mc.position, FieldOrientation::Normal);
        overlay_at(canvas, &img, pos.x, pos.y);
        Ok(())
    }
}

fn get_ttf_font_data(name: &str) -> &'static [u8] {
    match name {
        "0" => FONT_HELVETICA,
        "B" => FONT_DEJAVU_BOLD,
        "GS" => FONT_GS,
        _ => FONT_DEJAVU_MONO,
    }
}

fn measure_text_width(text: &str, font: &FontRef, scale: PxScale) -> f32 {
    use ab_glyph::{Font, ScaleFont};
    let scaled = font.as_scaled(scale);
    let mut width = 0.0f32;
    let mut prev = None;
    for ch in text.chars() {
        let glyph_id = font.glyph_id(ch);
        if let Some(prev_id) = prev {
            width += scaled.kern(prev_id, glyph_id);
        }
        width += scaled.h_advance(glyph_id);
        prev = Some(glyph_id);
    }
    width
}

fn word_wrap(text: &str, font: &FontRef, scale: PxScale, max_width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    for line in text.split('\n') {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut current_line = words[0].to_string();
        for word in &words[1..] {
            let test = format!("{} {}", current_line, word);
            let w = measure_text_width(&test, font, scale);
            if w > max_width {
                lines.push(current_line);
                current_line = word.to_string();
            } else {
                current_line = test;
            }
        }
        lines.push(current_line);
    }
    lines
}

fn get_text_top_left_pos(text: &TextField, w: f64, h: f64, state: &DrawerState) -> (f64, f64) {
    let (x, y) = state.get_text_position(text);

    if !text.position.calculate_from_bottom {
        return match text.font.orientation {
            FieldOrientation::Rotated90 => (x + h / 4.0, y),
            FieldOrientation::Rotated180 => (x + w, y + h / 4.0),
            FieldOrientation::Rotated270 => (x + 3.0 * h / 4.0, y + w),
            _ => (x, y + 3.0 * h / 4.0),
        };
    }

    let lines = if let Some(ref block) = text.block { block.max_lines.max(1) as f64 } else { 1.0 };
    let spacing = if let Some(ref block) = text.block { block.line_spacing as f64 } else { 0.0 };
    let offset = (lines - 1.0) * (h + spacing);

    match text.font.orientation {
        FieldOrientation::Rotated90 => (x + offset, y),
        FieldOrientation::Rotated180 => (x, y + offset),
        FieldOrientation::Rotated270 => (x - offset, y),
        _ => (x, y - offset),
    }
}

fn line_color_to_rgba(color: LineColor) -> Rgba<u8> {
    match color {
        LineColor::Black => Rgba([0, 0, 0, 255]),
        LineColor::White => Rgba([255, 255, 255, 255]),
    }
}

fn draw_filled_rect(canvas: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32, color: Rgba<u8>) {
    for py in y.max(0)..(y + h).min(canvas.height() as i32) {
        for px in x.max(0)..(x + w).min(canvas.width() as i32) {
            canvas.put_pixel(px as u32, py as u32, color);
        }
    }
}

fn adjust_image_typeset_position(
    img: &RgbaImage,
    pos: &LabelPosition,
    ori: FieldOrientation,
) -> LabelPosition {
    if !pos.calculate_from_bottom {
        return pos.clone();
    }

    let width = img.width() as i32;
    let height = img.height() as i32;
    let mut x = pos.x;
    let mut y = pos.y;

    match ori {
        FieldOrientation::Normal => y = (y - height).max(0),
        FieldOrientation::Rotated180 => x -= width,
        FieldOrientation::Rotated270 => {
            x = (x - height).max(0);
            y -= width;
        }
        _ => {}
    }

    LabelPosition {
        x,
        y,
        calculate_from_bottom: false,
        automatic_position: false,
    }
}

fn overlay_at(canvas: &mut RgbaImage, img: &RgbaImage, x: i32, y: i32) {
    for iy in 0..img.height() {
        for ix in 0..img.width() {
            let px = x + ix as i32;
            let py = y + iy as i32;
            if px >= 0 && py >= 0 && (px as u32) < canvas.width() && (py as u32) < canvas.height() {
                let pixel = *img.get_pixel(ix, iy);
                if pixel[3] > 0 {
                    canvas.put_pixel(px as u32, py as u32, pixel);
                }
            }
        }
    }
}

fn overlay_with_rotation(
    canvas: &mut RgbaImage,
    img: &RgbaImage,
    pos: &LabelPosition,
    orientation: FieldOrientation,
) {
    match orientation {
        FieldOrientation::Normal => {
            overlay_at(canvas, img, pos.x, pos.y);
        }
        FieldOrientation::Rotated90 => {
            let rotated = rotate_90(img);
            overlay_at(canvas, &rotated, pos.x, pos.y);
        }
        FieldOrientation::Rotated180 => {
            let rotated = rotate_180(img);
            overlay_at(canvas, &rotated, pos.x, pos.y);
        }
        FieldOrientation::Rotated270 => {
            let rotated = rotate_270(img);
            overlay_at(canvas, &rotated, pos.x, pos.y);
        }
    }
}

fn rotate_90(img: &RgbaImage) -> RgbaImage {
    let (w, h) = (img.width(), img.height());
    let mut out = RgbaImage::new(h, w);
    for y in 0..h {
        for x in 0..w {
            out.put_pixel(h - 1 - y, x, *img.get_pixel(x, y));
        }
    }
    out
}

fn rotate_180(img: &RgbaImage) -> RgbaImage {
    let (w, h) = (img.width(), img.height());
    let mut out = RgbaImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            out.put_pixel(w - 1 - x, h - 1 - y, *img.get_pixel(x, y));
        }
    }
    out
}

fn rotate_270(img: &RgbaImage) -> RgbaImage {
    let (w, h) = (img.width(), img.height());
    let mut out = RgbaImage::new(h, w);
    for y in 0..h {
        for x in 0..w {
            out.put_pixel(y, w - 1 - x, *img.get_pixel(x, y));
        }
    }
    out
}
