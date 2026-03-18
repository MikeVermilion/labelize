use image::{RgbaImage, Rgba};

/// MaxiCode is a 2D barcode with a fixed 30x33 hexagonal grid and a central bullseye.
/// This is a simplified implementation that encodes data into a recognizable MaxiCode pattern.
pub fn encode(content: &str) -> Result<RgbaImage, String> {
    if content.is_empty() {
        return Err("MaxiCode: empty content".to_string());
    }

    let data_bytes = content.as_bytes();

    // MaxiCode is always ~1 inch square. Module size is about 35 mils.
    // Grid is 30 columns x 33 rows of hexagonal modules
    let cols = 30usize;
    let rows = 33usize;

    // Module dimensions in pixels
    let mod_w = 8u32; // pixel width per module
    let mod_h = 7u32; // pixel height per module (hex)

    let img_width = cols as u32 * mod_w + mod_w;
    let img_height = rows as u32 * mod_h + mod_h;

    let mut img = RgbaImage::from_pixel(img_width, img_height, Rgba([255, 255, 255, 255]));
    let black = Rgba([0, 0, 0, 255]);

    // Draw central bullseye (3 concentric circles)
    let cx = img_width / 2;
    let cy = img_height / 2;
    draw_ring(&mut img, cx, cy, 18, 22, black);
    draw_ring(&mut img, cx, cy, 10, 14, black);
    draw_ring(&mut img, cx, cy, 2, 6, black);

    // Encode data into hex grid
    let mut data_bits: Vec<bool> = Vec::new();
    for &b in data_bytes {
        for shift in (0..6).rev() {
            data_bits.push((b >> shift) & 1 == 1);
        }
    }

    let mut bit_idx = 0;
    for row in 0..rows {
        for col in 0..cols {
            // Skip bullseye area (center region)
            let center_row = rows / 2;
            let center_col = cols / 2;
            if (row as i32 - center_row as i32).unsigned_abs() < 5
                && (col as i32 - center_col as i32).unsigned_abs() < 5
            {
                continue;
            }

            let val = if bit_idx < data_bits.len() {
                data_bits[bit_idx]
            } else {
                (row + col) % 3 == 0
            };
            bit_idx += 1;

            if val {
                let x_offset = if row % 2 == 1 { mod_w / 2 } else { 0 };
                let px = col as u32 * mod_w + x_offset + mod_w / 2;
                let py = row as u32 * mod_h + mod_h / 2;
                draw_hexagon(&mut img, px, py, mod_w / 2, black);
            }
        }
    }

    Ok(img)
}

fn draw_ring(img: &mut RgbaImage, cx: u32, cy: u32, inner_r: u32, outer_r: u32, color: Rgba<u8>) {
    let r2_outer = (outer_r * outer_r) as i64;
    let r2_inner = (inner_r * inner_r) as i64;
    let d = outer_r as i32;
    for dy in -d..=d {
        for dx in -d..=d {
            let dist2 = (dx as i64 * dx as i64 + dy as i64 * dy as i64) as i64;
            if dist2 >= r2_inner && dist2 <= r2_outer {
                let px = cx as i32 + dx;
                let py = cy as i32 + dy;
                if px >= 0 && py >= 0 && (px as u32) < img.width() && (py as u32) < img.height() {
                    img.put_pixel(px as u32, py as u32, color);
                }
            }
        }
    }
}

fn draw_hexagon(img: &mut RgbaImage, cx: u32, cy: u32, r: u32, color: Rgba<u8>) {
    // Approximate hexagonal module as a filled circle
    let r2 = (r * r) as i64;
    let ri = r as i32;
    for dy in -ri..=ri {
        for dx in -ri..=ri {
            if (dx as i64 * dx as i64 + dy as i64 * dy as i64) <= r2 {
                let px = cx as i32 + dx;
                let py = cy as i32 + dy;
                if px >= 0 && py >= 0 && (px as u32) < img.width() && (py as u32) < img.height() {
                    img.put_pixel(px as u32, py as u32, color);
                }
            }
        }
    }
}
