use image::{RgbaImage, Rgba};
use rxing::{BarcodeFormat, EncodeHints, Writer, EncodeHintValue};
use rxing::aztec::AztecWriter;

/// Generate an Aztec barcode image using rxing's proper encoder.
pub fn encode(content: &str, magnification: i32, error_correction: i32) -> Result<RgbaImage, String> {
    if content.is_empty() {
        return Err("Aztec: empty content".to_string());
    }

    let mag = magnification.max(1) as u32;
    let ec_percent = if error_correction > 0 { error_correction } else { 23 };

    // Use rxing Aztec writer to get proper bit matrix
    let writer = AztecWriter;
    let mut hints = EncodeHints::default();
    hints = hints.with(EncodeHintValue::ErrorCorrection(ec_percent.to_string()));
    hints = hints.with(EncodeHintValue::Margin("0".to_string()));

    // Encode with minimum size, we'll apply magnification ourselves
    let bit_matrix = writer
        .encode_with_hints(content, &BarcodeFormat::AZTEC, 0, 0, &hints)
        .map_err(|e| format!("Aztec encoding failed: {}", e))?;

    let bm_width = bit_matrix.getWidth() as u32;
    let bm_height = bit_matrix.getHeight() as u32;

    // Render at magnification
    let img_width = bm_width * mag;
    let img_height = bm_height * mag;

    let mut img = RgbaImage::from_pixel(img_width, img_height, Rgba([255, 255, 255, 255]));
    let black = Rgba([0, 0, 0, 255]);

    for y in 0..bm_height {
        for x in 0..bm_width {
            if bit_matrix.get(x, y) {
                let px = x * mag;
                let py = y * mag;
                for dy in 0..mag {
                    for dx in 0..mag {
                        img.put_pixel(px + dx, py + dy, black);
                    }
                }
            }
        }
    }

    Ok(img)
}
