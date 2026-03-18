use image::{RgbaImage, Rgba};

/// Generate a PDF417 barcode image.
/// This is a simplified implementation that produces a recognizable PDF417-style pattern.
pub fn encode(content: &str, height: i32, column_count: i32, row_count: i32, _truncated: bool) -> Result<RgbaImage, String> {
    if content.is_empty() {
        return Err("PDF417: empty content".to_string());
    }

    let data_bytes = content.as_bytes();
    let cols = column_count.max(1) as usize;
    let rows = if row_count > 0 {
        row_count as usize
    } else {
        ((data_bytes.len() + cols - 1) / cols).max(3)
    };

    let row_height = height.max(1) as u32;

    // PDF417 has start/stop patterns and data columns
    // Each column is 17 modules wide, start pattern is 17, stop is 18
    let module_width = 17 * cols + 17 + 18 + 17 * 2; // start + left_row_ind + data + right_row_ind + stop
    let quiet_zone = 10;
    let total_width = (module_width + quiet_zone * 2) as u32;
    let total_height = rows as u32 * row_height + quiet_zone as u32 * 2;

    let mut img = RgbaImage::from_pixel(total_width, total_height, Rgba([255, 255, 255, 255]));
    let black = Rgba([0, 0, 0, 255]);

    let start_pattern: [u8; 17] = [1,1,1,1,1,1,1,1,0,1,0,1,0,1,0,0,0];
    let stop_pattern: [u8; 18] = [1,1,1,1,1,1,1,0,1,0,0,0,1,0,1,0,0,1];

    for row in 0..rows {
        let y_start = quiet_zone as u32 + row as u32 * row_height;
        let mut x = quiet_zone as u32;

        // Start pattern
        for &bit in &start_pattern {
            if bit == 1 {
                for dy in 0..row_height {
                    if x < total_width && y_start + dy < total_height {
                        img.put_pixel(x, y_start + dy, black);
                    }
                }
            }
            x += 1;
        }

        // Left row indicator (simplified)
        for i in 0..17u32 {
            let is_bar = (i + row as u32) % 4 < 2;
            if is_bar {
                for dy in 0..row_height {
                    if x < total_width && y_start + dy < total_height {
                        img.put_pixel(x, y_start + dy, black);
                    }
                }
            }
            x += 1;
        }

        // Data columns
        for col in 0..cols {
            let byte_idx = row * cols + col;
            let byte_val = if byte_idx < data_bytes.len() {
                data_bytes[byte_idx]
            } else {
                0x90 // padding codeword
            };

            // Encode byte as 17-module pattern
            for i in 0..17u32 {
                let bit = (byte_val >> (i % 8)) & 1;
                let is_bar = if i < 8 { bit == 1 } else { (i + row as u32) % 3 == 0 };
                if is_bar {
                    for dy in 0..row_height {
                        if x < total_width && y_start + dy < total_height {
                            img.put_pixel(x, y_start + dy, black);
                        }
                    }
                }
                x += 1;
            }
        }

        // Right row indicator (simplified)
        for i in 0..17u32 {
            let is_bar = (i + row as u32 + 1) % 4 < 2;
            if is_bar {
                for dy in 0..row_height {
                    if x < total_width && y_start + dy < total_height {
                        img.put_pixel(x, y_start + dy, black);
                    }
                }
            }
            x += 1;
        }

        // Stop pattern
        for &bit in &stop_pattern {
            if bit == 1 {
                for dy in 0..row_height {
                    if x < total_width && y_start + dy < total_height {
                        img.put_pixel(x, y_start + dy, black);
                    }
                }
            }
            x += 1;
        }
    }

    Ok(img)
}
