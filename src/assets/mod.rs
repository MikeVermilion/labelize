pub static FONT_HELVETICA_BOLD: &[u8] = include_bytes!("fonts/HelveticaBoldCondensedCustom.ttf");
#[cfg(not(all(feature = "wasm-bindings", target_arch = "wasm32")))]
pub static FONT_DEJAVU_SANS_MONO: &[u8] = include_bytes!("fonts/DejaVuSansMono.ttf");
#[cfg(not(all(feature = "wasm-bindings", target_arch = "wasm32")))]
pub static FONT_DEJAVU_SANS_MONO_BOLD: &[u8] = include_bytes!("fonts/DejaVuSansMonoBold.ttf");
pub static FONT_ZPL_GS: &[u8] = include_bytes!("fonts/ZplGSCustom.ttf");
