use wasm_bindgen::prelude::*;

use crate::{zpl_to_png, DrawerOptions};

#[wasm_bindgen]
pub fn render_zpl_to_png(
    zpl: &str,
    width_mm: Option<f64>,
    height_mm: Option<f64>,
    dpmm: Option<i32>,
    enable_inverted_labels: Option<bool>,
) -> Result<Vec<u8>, JsValue> {
    let mut options = DrawerOptions::default();

    if let Some(width_mm) = width_mm {
        options.label_width_mm = width_mm;
    }
    if let Some(height_mm) = height_mm {
        options.label_height_mm = height_mm;
    }
    if let Some(dpmm) = dpmm {
        options.dpmm = dpmm;
    }
    if let Some(enable_inverted_labels) = enable_inverted_labels {
        options.enable_inverted_labels = enable_inverted_labels;
    }

    zpl_to_png(zpl.as_bytes(), options).map_err(|err| JsValue::from_str(&err.to_string()))
}
