use image::{RgbaImage, Rgba};
use datamatrix::{DataMatrix, SymbolList};

/// Generate a Data Matrix barcode image using a proper ECC 200 encoder.
pub fn encode(content: &str, magnification: i32) -> Result<RgbaImage, String> {
    if content.is_empty() {
        return Err("DataMatrix: empty content".to_string());
    }

    let mag = magnification.max(1) as u32;

    let code = DataMatrix::encode(content.as_bytes(), SymbolList::default())
        .map_err(|e| format!("DataMatrix encoding failed: {:?}", e))?;

    let bitmap = code.bitmap();
    let bm_width = bitmap.width() as u32;
    let bm_height = bitmap.height() as u32;

    // Render to image (no quiet zone — Labelary omits it)
    let img_width = bm_width * mag;
    let img_height = bm_height * mag;
    let mut img = RgbaImage::from_pixel(img_width, img_height, Rgba([255, 255, 255, 255]));
    let black = Rgba([0, 0, 0, 255]);

    // pixels() yields (x, y) for each dark module
    for (col, row) in bitmap.pixels() {
        let px = col as u32 * mag;
        let py = row as u32 * mag;
        for dy in 0..mag {
            for dx in 0..mag {
                if px + dx < img_width && py + dy < img_height {
                    img.put_pixel(px + dx, py + dy, black);
                }
            }
        }
    }

    Ok(img)
}
