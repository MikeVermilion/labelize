use image::{RgbaImage, Rgba};

/// Generate a Data Matrix barcode image.
/// Simplified implementation producing a recognizable Data Matrix pattern.
pub fn encode(content: &str, magnification: i32) -> Result<RgbaImage, String> {
    if content.is_empty() {
        return Err("DataMatrix: empty content".to_string());
    }

    let mag = magnification.max(1) as u32;
    let data_bytes = content.as_bytes();

    // Determine matrix size (always even, minimum 10x10)
    let needed = data_bytes.len();
    let side = if needed <= 3 { 10 }
        else if needed <= 6 { 12 }
        else if needed <= 10 { 14 }
        else if needed <= 16 { 16 }
        else if needed <= 25 { 18 }
        else if needed <= 36 { 20 }
        else if needed <= 49 { 22 }
        else if needed <= 64 { 24 }
        else if needed <= 100 { 26 }
        else { ((needed as f64).sqrt().ceil() as usize + 1) & !1 }; // round up to even
    let side = side.max(10);

    let mut modules = vec![vec![false; side]; side];

    // Draw L-shaped finder pattern (bottom and left edges are solid)
    for i in 0..side {
        modules[side - 1][i] = true; // bottom row solid
        modules[i][0] = true;        // left column solid
    }

    // Draw clock track (top and right edges are alternating)
    for i in 0..side {
        modules[0][i] = i % 2 == 0;        // top row alternating
        modules[i][side - 1] = i % 2 == 0;  // right column alternating (even = solid going up)
    }

    // Encode data inside the matrix
    let mut data_bits: Vec<bool> = Vec::new();
    for &b in data_bytes {
        for shift in (0..8).rev() {
            data_bits.push((b >> shift) & 1 == 1);
        }
    }

    let mut bit_idx = 0;
    for row in 1..side - 1 {
        for col in 1..side - 1 {
            let val = if bit_idx < data_bits.len() {
                data_bits[bit_idx]
            } else {
                (row + col) % 2 == 0 // padding pattern
            };
            modules[row][col] = val;
            bit_idx += 1;
        }
    }

    // Render to image
    let quiet_zone = 2;
    let img_side = (side + quiet_zone * 2) as u32 * mag;
    let mut img = RgbaImage::from_pixel(img_side, img_side, Rgba([255, 255, 255, 255]));
    let black = Rgba([0, 0, 0, 255]);

    for row in 0..side {
        for col in 0..side {
            if modules[row][col] {
                let px = (col + quiet_zone) as u32 * mag;
                let py = (row + quiet_zone) as u32 * mag;
                for dy in 0..mag {
                    for dx in 0..mag {
                        if px + dx < img_side && py + dy < img_side {
                            img.put_pixel(px + dx, py + dy, black);
                        }
                    }
                }
            }
        }
    }

    Ok(img)
}
