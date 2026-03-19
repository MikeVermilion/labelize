use image::{RgbaImage, Rgba};
use pdf417::{PDF417, PDF417Encoder};

/// Calculate the number of codewords needed for byte encoding.
/// This mirrors the internal logic of PDF417Encoder::append_bytes.
fn byte_encoding_codewords(data_len: usize) -> usize {
    // 1 for the length indicator (slot 0, used internally)
    // 1 for LATCH_BYTE/LATCH_BYTE_M6 mode switch
    // floor(data_len/6) * 5 for packed groups
    // data_len % 6 for remaining bytes
    let packed_groups = data_len / 6;
    let remaining = data_len % 6;
    1 + 1 + packed_groups * 5 + remaining
}

/// Generate a PDF417 barcode image using a proper encoder.
pub fn encode(content: &str, row_height: i32, security_level: i32, column_count: i32, row_count: i32, truncated: bool) -> Result<RgbaImage, String> {
    if content.is_empty() {
        return Err("PDF417: empty content".to_string());
    }

    let cols = column_count.max(1).min(30) as u8;
    let scale_y = row_height.max(1) as u32;
    let data_bytes = content.as_bytes();

    // Calculate exact codewords needed for the data
    let data_cws = byte_encoding_codewords(data_bytes.len());

    // Determine security level and ECC codeword count
    let sec_level = if security_level > 0 && security_level <= 8 {
        security_level as u8
    } else {
        // Will be auto-selected later, estimate with level 2 for row calculation
        2
    };
    let ecc_cws = pdf417::ecc::ecc_count(sec_level);
    let total_cws_needed = data_cws + ecc_cws;

    // Calculate rows needed
    let min_rows = ((total_cws_needed + cols as usize - 1) / cols as usize).max(3) as u8;
    let rows = if row_count > 0 {
        (row_count as u8).max(min_rows).min(90)
    } else {
        min_rows.min(90)
    };

    let capacity = (cols as usize) * (rows as usize);
    let mut codewords = vec![0u16; capacity];

    let encoder = PDF417Encoder::new(&mut codewords, false)
        .append_bytes(data_bytes);

    let (level, sealed) = if security_level > 0 && security_level <= 8 {
        let level = security_level as u8;
        let s = encoder.seal(level);
        (level, s)
    } else {
        encoder.fit_seal()
            .ok_or_else(|| "PDF417: data too large for configuration".to_string())?
    };

    encode_to_image(sealed, rows, cols, level, truncated, scale_y)
}

fn encode_to_image(codewords: &[u16], rows: u8, cols: u8, level: u8, truncated: bool, scale_y: u32) -> Result<RgbaImage, String> {
    // Calculate pixel dimensions (no quiet zone — matches Labelary)
    let img_width = if truncated {
        (pdf417::START_PATTERN_LEN as usize + 17 + cols as usize * 17 + 1) as u32
    } else {
        (pdf417::START_PATTERN_LEN as usize + 17 + cols as usize * 17 + 17 + pdf417::END_PATTERN_LEN as usize) as u32
    };
    let img_height = rows as u32 * scale_y;

    // Render to bool buffer
    let buf_size = img_width as usize * img_height as usize;
    let mut storage = vec![false; buf_size];
    let pdf = PDF417::new(codewords, rows, cols, level)
        .truncated(truncated)
        .scaled((1, scale_y));
    pdf.render(&mut storage[..]);

    // Convert to RGBA image
    let mut img = RgbaImage::from_pixel(img_width, img_height, Rgba([255, 255, 255, 255]));
    let black = Rgba([0, 0, 0, 255]);

    for (idx, &is_dark) in storage.iter().enumerate() {
        if is_dark {
            let x = (idx as u32) % img_width;
            let y = (idx as u32) / img_width;
            if x < img_width && y < img_height {
                img.put_pixel(x, y, black);
            }
        }
    }

    Ok(img)
}
