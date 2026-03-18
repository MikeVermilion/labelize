use image::RgbaImage;
use std::io::Write;
use crate::error::LabelizeError;

pub fn encode_png(img: &RgbaImage, w: &mut impl Write) -> Result<(), LabelizeError> {
    let (width, height) = img.dimensions();
    let mut gray = image::GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let val = if pixel[0] > 128 { 255u8 } else { 0u8 };
            gray.put_pixel(x, y, image::Luma([val]));
        }
    }

    let encoder = image::codecs::png::PngEncoder::new(w);
    use image::ImageEncoder;
    encoder
        .write_image(
            gray.as_raw(),
            width,
            height,
            image::ExtendedColorType::L8,
        )
        .map_err(|e| LabelizeError::Encode(format!("PNG encode error: {}", e)))
}
