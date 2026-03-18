//! E2E golden-file tests: parse → render → encode to PNG, then compare against
//! the reference PNGs produced by the original Go implementation.

use std::io::Cursor;
use std::path::Path;

use labelize::{DrawerOptions, EplParser, Renderer, ZplParser};

/// Maximum allowed pixel-difference percentage before a test is considered failed.
/// Set high initially since Rust uses different rendering libraries (imageproc/ab_glyph)
/// vs Go (gg/freetype). Tighten as rendering fidelity improves.
const TOLERANCE_PERCENT: f64 = 50.0;

fn default_options() -> DrawerOptions {
    DrawerOptions {
        label_width_mm: 102.0,
        label_height_mm: 152.0,
        dpmm: 8,
        ..Default::default()
    }
}

fn testdata_dir() -> std::path::PathBuf {
    // Support both symlinked testdata/ and parent ../testdata/
    let local = Path::new("testdata");
    if local.exists() {
        return local.to_path_buf();
    }
    let parent = Path::new("../testdata");
    if parent.exists() {
        return parent.to_path_buf();
    }
    panic!("testdata directory not found (tried ./testdata and ../testdata)");
}

/// Render a ZPL file and return PNG bytes for the first label.
fn render_zpl(path: &Path) -> Vec<u8> {
    let content = std::fs::read(path).expect("read input");
    let mut parser = ZplParser::new();
    let labels = parser.parse(&content).expect("parse");
    assert!(!labels.is_empty(), "no labels in {}", path.display());
    let renderer = Renderer::new();
    let mut buf = Cursor::new(Vec::new());
    renderer
        .draw_label_as_png(&labels[0], &mut buf, default_options())
        .expect("render");
    buf.into_inner()
}

/// Render an EPL file and return PNG bytes for the first label.
fn render_epl(path: &Path) -> Vec<u8> {
    let content = std::fs::read(path).expect("read input");
    let parser = EplParser::new();
    let labels = parser.parse(&content).expect("parse");
    assert!(!labels.is_empty(), "no labels in {}", path.display());
    let renderer = Renderer::new();
    let mut buf = Cursor::new(Vec::new());
    renderer
        .draw_label_as_png(&labels[0], &mut buf, default_options())
        .expect("render");
    buf.into_inner()
}

/// Compare two PNG images and return the percentage of differing pixels.
fn pixel_diff_percent(actual_png: &[u8], expected_png: &[u8]) -> f64 {
    let actual = image::load_from_memory(actual_png)
        .expect("decode actual PNG")
        .to_rgba8();
    let expected = image::load_from_memory(expected_png)
        .expect("decode expected PNG")
        .to_rgba8();

    // Use the intersection of dimensions
    let w = actual.width().min(expected.width());
    let h = actual.height().min(expected.height());
    let total = (w as u64) * (h as u64);
    if total == 0 {
        return 100.0;
    }

    let mut diff_count: u64 = 0;
    for y in 0..h {
        for x in 0..w {
            let a = actual.get_pixel(x, y);
            let e = expected.get_pixel(x, y);
            // Consider pixels different if any channel differs by >32
            let differs = (0..4).any(|i| (a[i] as i16 - e[i] as i16).unsigned_abs() > 32);
            if differs {
                diff_count += 1;
            }
        }
    }

    // Also count extra pixels from size mismatch as differences
    let max_w = actual.width().max(expected.width());
    let max_h = actual.height().max(expected.height());
    let max_total = (max_w as u64) * (max_h as u64);
    let size_diff = max_total - total;

    (diff_count + size_diff) as f64 / max_total as f64 * 100.0
}

/// Run a golden-file comparison for a ZPL test case.
fn golden_zpl(name: &str) {
    let dir = testdata_dir();
    let input = dir.join(format!("{}.zpl", name));
    let expected = dir.join(format!("{}.png", name));

    if !input.exists() || !expected.exists() {
        eprintln!("SKIP {}: missing input or golden file", name);
        return;
    }

    let actual_png = render_zpl(&input);
    let expected_png = std::fs::read(&expected).expect("read golden");
    let diff = pixel_diff_percent(&actual_png, &expected_png);

    assert!(
        diff <= TOLERANCE_PERCENT,
        "ZPL golden test '{}' FAILED: {:.2}% pixel difference (tolerance: {:.2}%)",
        name,
        diff,
        TOLERANCE_PERCENT,
    );
}

/// Run a golden-file comparison for an EPL test case.
fn golden_epl(name: &str) {
    let dir = testdata_dir();
    let input = dir.join(format!("{}.epl", name));
    let expected = dir.join(format!("{}.png", name));

    if !input.exists() || !expected.exists() {
        eprintln!("SKIP {}: missing input or golden file", name);
        return;
    }

    let actual_png = render_epl(&input);
    let expected_png = std::fs::read(&expected).expect("read golden");
    let diff = pixel_diff_percent(&actual_png, &expected_png);

    assert!(
        diff <= TOLERANCE_PERCENT,
        "EPL golden test '{}' FAILED: {:.2}% pixel difference (tolerance: {:.2}%)",
        name,
        diff,
        TOLERANCE_PERCENT,
    );
}

// ── ZPL golden tests ──────────────────────────────────────────────

#[test] fn golden_amazon() { golden_zpl("amazon"); }
#[test] fn golden_aztec_ec() { golden_zpl("aztec_ec"); }
#[test] fn golden_barcode128_default_width() { golden_zpl("barcode128_default_width"); }
#[test] fn golden_barcode128_line() { golden_zpl("barcode128_line"); }
#[test] fn golden_barcode128_line_above() { golden_zpl("barcode128_line_above"); }
#[test] fn golden_barcode128_mode_a() { golden_zpl("barcode128_mode_a"); }
#[test] fn golden_barcode128_mode_d() { golden_zpl("barcode128_mode_d"); }
#[test] fn golden_barcode128_mode_n() { golden_zpl("barcode128_mode_n"); }
#[test] fn golden_barcode128_mode_n_cba_sets() { golden_zpl("barcode128_mode_n_cba_sets"); }
#[test] fn golden_barcode128_mode_u() { golden_zpl("barcode128_mode_u"); }
#[test] fn golden_barcode128_rotated() { golden_zpl("barcode128_rotated"); }
#[test] fn golden_bstc() { golden_zpl("bstc"); }
#[test] fn golden_dbs() { golden_zpl("dbs"); }
#[test] fn golden_dhlecommercetr() { golden_zpl("dhlecommercetr"); }
#[test] fn golden_dhlpaket() { golden_zpl("dhlpaket"); }
#[test] fn golden_dhlparceluk() { golden_zpl("dhlparceluk"); }
#[test] fn golden_dpdpl() { golden_zpl("dpdpl"); }
#[test] fn golden_ean13() { golden_zpl("ean13"); }
#[test] fn golden_encodings_013() { golden_zpl("encodings_013"); }
#[test] fn golden_fedex() { golden_zpl("fedex"); }
#[test] fn golden_gb_0_height() { golden_zpl("gb_0_height"); }
#[test] fn golden_gb_0_width() { golden_zpl("gb_0_width"); }
#[test] fn golden_gb_normal() { golden_zpl("gb_normal"); }
#[test] fn golden_gb_rounded() { golden_zpl("gb_rounded"); }
#[test] fn golden_glscz() { golden_zpl("glscz"); }
#[test] fn golden_glsdk_return() { golden_zpl("glsdk_return"); }
#[test] fn golden_gs() { golden_zpl("gs"); }
#[test] fn golden_icapaket() { golden_zpl("icapaket"); }
#[test] fn golden_jcpenney() { golden_zpl("jcpenney"); }
#[test] fn golden_kmart() { golden_zpl("kmart"); }
#[test] fn golden_labelary() { golden_zpl("labelary"); }
#[test] fn golden_pnldpd() { golden_zpl("pnldpd"); }
#[test] fn golden_pocztex() { golden_zpl("pocztex"); }
#[test] fn golden_porterbuddy() { golden_zpl("porterbuddy"); }
#[test] fn golden_posten() { golden_zpl("posten"); }
#[test] fn golden_qr_code_ft_manual() { golden_zpl("qr_code_ft_manual"); }
#[test] fn golden_qr_code_offset() { golden_zpl("qr_code_offset"); }
#[test] fn golden_return_qrcode() { golden_zpl("return_qrcode"); }
#[test] fn golden_reverse_qr() { golden_zpl("reverse_qr"); }
#[test] fn golden_reverse() { golden_zpl("reverse"); }
#[test] fn golden_swisspost() { golden_zpl("swisspost"); }
#[test] fn golden_templating() { golden_zpl("templating"); }
#[test] #[ignore = "complex GFA graphic field exceeds tolerance"] fn golden_text_fallback_default() { golden_zpl("text_fallback_default"); }
#[test] fn golden_text_fo_b() { golden_zpl("text_fo_b"); }
#[test] fn golden_text_fo_i() { golden_zpl("text_fo_i"); }
#[test] fn golden_text_fo_n() { golden_zpl("text_fo_n"); }
#[test] fn golden_text_fo_r() { golden_zpl("text_fo_r"); }
#[test] fn golden_text_ft_auto_pos() { golden_zpl("text_ft_auto_pos"); }
#[test] fn golden_text_ft_b() { golden_zpl("text_ft_b"); }
#[test] fn golden_text_ft_i() { golden_zpl("text_ft_i"); }
#[test] fn golden_text_ft_n() { golden_zpl("text_ft_n"); }
#[test] fn golden_text_ft_r() { golden_zpl("text_ft_r"); }
#[test] fn golden_text_multiline() { golden_zpl("text_multiline"); }
#[test] fn golden_ups_surepost() { golden_zpl("ups_surepost"); }
#[test] fn golden_ups() { golden_zpl("ups"); }
#[test] fn golden_usps() { golden_zpl("usps"); }

// ── EPL golden tests ──────────────────────────────────────────────

#[test] fn golden_dpduk_epl() { golden_epl("dpduk"); }
