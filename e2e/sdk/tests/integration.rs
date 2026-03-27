use std::io::Cursor;
use std::fs;
use std::path::Path;
use labelize::{DrawerOptions, Renderer, ZplParser};

const SAMPLE_ZPL: &[u8] = b"^XA\
^CF0,40\
^FO50,50^FDHello Labelize E2E^FS\
^BY3,2,100\
^FO50,120^BC^FD12345678^FS\
^FO50,280^GB300,3,3^FS\
^FO50,310^A0N,30,30^FDTest Label^FS\
^XZ";

const OUTPUT_DIR: &str = "../sdk-output";

fn default_options() -> DrawerOptions {
    DrawerOptions {
        label_width_mm: 102.0,
        label_height_mm: 152.0,
        dpmm: 8,
        ..Default::default()
    }
}

fn save_output(name: &str, data: &[u8]) {
    let dir = Path::new(OUTPUT_DIR);
    fs::create_dir_all(dir).ok();
    fs::write(dir.join(name), data).expect("failed to save output");
}

#[test]
fn parse_zpl_produces_labels() {
    let mut parser = ZplParser::new();
    let labels = parser.parse(SAMPLE_ZPL).expect("ZPL parse failed");
    assert!(!labels.is_empty(), "Expected at least one label");
}

#[test]
fn render_png_produces_valid_output() {
    let mut parser = ZplParser::new();
    let labels = parser.parse(SAMPLE_ZPL).expect("ZPL parse failed");
    let label = &labels[0];

    let renderer = Renderer::new();
    let mut buf = Cursor::new(Vec::new());
    renderer
        .draw_label_as_png(label, &mut buf, default_options())
        .expect("PNG render failed");

    let bytes = buf.into_inner();
    assert!(bytes.len() > 100, "PNG output too small: {} bytes", bytes.len());

    // Verify PNG signature (magic bytes)
    assert_eq!(&bytes[0..4], &[0x89, 0x50, 0x4E, 0x47], "Invalid PNG header");

    save_output("sample.png", &bytes);
}

#[test]
fn render_pdf_produces_valid_output() {
    let mut parser = ZplParser::new();
    let labels = parser.parse(SAMPLE_ZPL).expect("ZPL parse failed");
    let label = &labels[0];
    let options = default_options();

    let renderer = Renderer::new();
    let mut png_buf = Cursor::new(Vec::new());
    renderer
        .draw_label_as_png(label, &mut png_buf, options.clone())
        .expect("PNG render failed");

    let img = image::load_from_memory(&png_buf.into_inner())
        .expect("image decode failed")
        .to_rgba8();

    let mut pdf_buf = Cursor::new(Vec::new());
    labelize::encode_pdf(&img, &options, &mut pdf_buf).expect("PDF encode failed");

    let bytes = pdf_buf.into_inner();
    assert!(bytes.len() > 100, "PDF output too small: {} bytes", bytes.len());

    // Verify PDF signature
    assert_eq!(&bytes[0..5], b"%PDF-", "Invalid PDF header");

    save_output("sample.pdf", &bytes);
}

#[test]
fn custom_dimensions_render() {
    let mut parser = ZplParser::new();
    let labels = parser.parse(SAMPLE_ZPL).expect("ZPL parse failed");
    let label = &labels[0];

    let options = DrawerOptions {
        label_width_mm: 100.0,
        label_height_mm: 62.0,
        dpmm: 12,
        ..Default::default()
    };

    let renderer = Renderer::new();
    let mut buf = Cursor::new(Vec::new());
    renderer
        .draw_label_as_png(label, &mut buf, options)
        .expect("PNG render with custom dims failed");

    let bytes = buf.into_inner();
    assert!(bytes.len() > 100, "PNG output too small: {} bytes", bytes.len());

    save_output("sample-custom.png", &bytes);
}

#[test]
fn multiple_labels_in_one_input() {
    let multi_zpl = b"^XA^FO50,50^FDLabel 1^FS^XZ\n^XA^FO50,50^FDLabel 2^FS^XZ";
    let mut parser = ZplParser::new();
    let labels = parser.parse(multi_zpl).expect("ZPL parse failed");
    assert_eq!(labels.len(), 2, "Expected 2 labels, got {}", labels.len());

    let renderer = Renderer::new();
    let options = default_options();
    for (i, label) in labels.iter().enumerate() {
        let mut buf = Cursor::new(Vec::new());
        renderer
            .draw_label_as_png(label, &mut buf, options.clone())
            .expect(&format!("Render label {} failed", i));
        let bytes = buf.into_inner();
        assert!(bytes.len() > 100);
        save_output(&format!("multi-label-{}.png", i), &bytes);
    }
}
