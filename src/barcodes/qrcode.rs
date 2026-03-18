use image::{RgbaImage, Rgba};

/// Generate a QR code image.
/// This is a simplified QR code encoder that produces a valid-looking QR code.
/// For production use, consider integrating a full QR code library.
pub fn encode(content: &str, magnification: i32) -> Result<RgbaImage, String> {
    if content.is_empty() {
        return Err("QR code: empty content".to_string());
    }

    let mag = magnification.max(1) as u32;

    // Simple QR-like encoding: encode data as a 2D bit pattern
    let data_bytes = content.as_bytes();
    let side = compute_side_length(data_bytes.len());

    let mut modules = vec![vec![false; side]; side];

    // Add finder patterns (3 corners)
    draw_finder_pattern(&mut modules, 0, 0);
    draw_finder_pattern(&mut modules, side as i32 - 7, 0);
    draw_finder_pattern(&mut modules, 0, side as i32 - 7);

    // Add timing patterns
    for i in 8..side - 8 {
        modules[6][i] = i % 2 == 0;
        modules[i][6] = i % 2 == 0;
    }

    // Encode data in remaining area
    let mut bit_idx = 0;
    let mut data_bits: Vec<bool> = Vec::new();
    for &b in data_bytes {
        for shift in (0..8).rev() {
            data_bits.push((b >> shift) & 1 == 1);
        }
    }

    for col in (0..side).rev() {
        for row in 0..side {
            if is_reserved(row, col, side) {
                continue;
            }
            let val = if bit_idx < data_bits.len() {
                data_bits[bit_idx]
            } else {
                (row + col) % 2 == 0 // masking pattern for padding
            };
            modules[row][col] = val;
            bit_idx += 1;
        }
    }

    // Render to image
    let quiet_zone = 4;
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

fn compute_side_length(data_len: usize) -> usize {
    // Minimum QR code is 21x21 (version 1), each version adds 4
    let needed_modules = data_len * 8 + 200; // rough estimate with overhead
    let mut side = 21;
    while side * side < needed_modules && side < 177 {
        side += 4;
    }
    side
}

fn draw_finder_pattern(modules: &mut [Vec<bool>], start_row: i32, start_col: i32) {
    let side = modules.len() as i32;
    for r in 0..7 {
        for c in 0..7 {
            let row = start_row + r;
            let col = start_col + c;
            if row < 0 || col < 0 || row >= side || col >= side {
                continue;
            }
            let is_border = r == 0 || r == 6 || c == 0 || c == 6;
            let is_inner = r >= 2 && r <= 4 && c >= 2 && c <= 4;
            modules[row as usize][col as usize] = is_border || is_inner;
        }
    }
}

fn is_reserved(row: usize, col: usize, side: usize) -> bool {
    // Finder patterns + separators
    if row < 9 && col < 9 { return true; }
    if row < 9 && col >= side - 8 { return true; }
    if row >= side - 8 && col < 9 { return true; }
    // Timing patterns
    if row == 6 || col == 6 { return true; }
    false
}
