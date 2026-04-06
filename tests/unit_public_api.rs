use labelize::{parse_zpl, render_label_to_png, zpl_to_png, DrawerOptions};

fn default_options() -> DrawerOptions {
    DrawerOptions {
        label_width_mm: 101.625,
        label_height_mm: 203.25,
        dpmm: 8,
        ..Default::default()
    }
}

fn decode_png(png: &[u8]) -> image::GrayImage {
    image::load_from_memory(png)
        .expect("decode PNG")
        .to_luma8()
}

#[test]
fn parse_zpl_returns_labels() {
    let labels = parse_zpl(b"^XA^FO20,20^GB40,30,2^FS^XZ").expect("parse ZPL");
    assert_eq!(labels.len(), 1);
}

#[test]
fn render_label_to_png_produces_png_bytes() {
    let labels = parse_zpl(b"^XA^FO20,20^GB40,30,2^FS^XZ").expect("parse ZPL");
    let png = render_label_to_png(&labels[0], default_options()).expect("render PNG");
    let img = decode_png(&png);

    assert!(img.width() > 0);
    assert!(img.height() > 0);
}

#[test]
fn zpl_to_png_renders_first_label() {
    let png = zpl_to_png(b"^XA^FO20,20^FDHello^FS^XZ", default_options()).expect("render ZPL");
    let img = decode_png(&png);
    let has_dark_pixels = img.pixels().any(|pixel| pixel[0] == 0);

    assert!(has_dark_pixels, "expected rendered content");
}
