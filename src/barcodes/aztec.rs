use image::{RgbaImage, Rgba};

/// Generate an Aztec barcode image.
/// Simplified implementation that produces a recognizable Aztec-style pattern.
pub fn encode(content: &str, magnification: i32, error_correction: i32) -> Result<RgbaImage, String> {
    if content.is_empty() {
        return Err("Aztec: empty content".to_string());
    }

    let mag = magnification.max(1) as u32;
    let _ec = error_correction.max(0);

    let data_bytes = content.as_bytes();

    // Compute size based on data length
    let data_bits = data_bytes.len() * 8;
    let layers = ((data_bits as f64).sqrt() / 4.0).ceil() as usize;
    let layers = layers.max(1).min(32);

    // Compact Aztec: bullseye is 9x9, full: 13x13
    let is_compact = layers <= 4;
    let bullseye_size = if is_compact { 9 } else { 13 };
    let side = bullseye_size + layers * 4;

    let mut modules = vec![vec![false; side]; side];
    let center = side / 2;

    // Draw bullseye (concentric squares)
    let rings = if is_compact { 2 } else { 3 };
    for ring in 0..rings {
        let r = ring * 2;
        let start = center - 1 - r;
        let end = center + 1 + r;
        // Draw square outline if ring is even (black), skip if odd
        if ring % 2 == 0 || ring == 0 {
            for i in start..=end {
                if i < side {
                    if start < side { modules[start][i] = true; }
                    if end < side { modules[end][i] = true; }
                    modules[i][start] = true;
                    modules[i][end] = true;
                }
            }
        }
    }

    // Center pixel
    modules[center][center] = true;

    // Draw orientation marks
    if center >= 5 && center + 5 < side {
        // Top-right corner mark
        modules[center - 5][center + 3] = true;
        modules[center - 5][center + 4] = true;
    }

    // Encode data in layers around the bullseye
    let mut bit_idx = 0;
    let mut data_bits_vec: Vec<bool> = Vec::new();
    for &b in data_bytes {
        for shift in (0..8).rev() {
            data_bits_vec.push((b >> shift) & 1 == 1);
        }
    }

    for layer in 0..layers {
        let offset = if is_compact { 5 } else { 7 };
        let ring_start = center as i32 - offset as i32 - (layer as i32 * 2) - 1;
        let ring_end = center as i32 + offset as i32 + (layer as i32 * 2) + 1;

        // Walk around the ring
        let positions = get_ring_positions(ring_start, ring_end, side);
        for (r, c) in positions {
            if bit_idx < data_bits_vec.len() {
                modules[r][c] = data_bits_vec[bit_idx];
            } else {
                modules[r][c] = (r + c) % 2 == 0;
            }
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

fn get_ring_positions(start: i32, end: i32, side: usize) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let s = start.max(0) as usize;
    let e = (end as usize).min(side - 1);

    // Top edge
    for c in s..=e { positions.push((s, c)); }
    // Right edge
    for r in s + 1..=e { positions.push((r, e)); }
    // Bottom edge
    for c in (s..e).rev() { positions.push((e, c)); }
    // Left edge
    for r in (s + 1..e).rev() { positions.push((r, s)); }

    positions
}
