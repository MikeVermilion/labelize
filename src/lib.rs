pub mod assets;
pub mod barcodes;
pub mod drawers;
pub mod elements;
pub mod encodings;
pub mod error;
pub mod hex;
pub mod images;
pub mod parsers;

pub use drawers::renderer::Renderer;
pub use elements::drawer_options::DrawerOptions;
pub use elements::label_info::LabelInfo;
pub use error::LabelizeError;
pub use images::monochrome::encode_png;
#[cfg(feature = "pdf")]
pub use images::pdf::encode_pdf;
pub use parsers::epl_parser::EplParser;
pub use parsers::zpl_parser::ZplParser;

pub fn parse_zpl(zpl_data: &[u8]) -> error::Result<Vec<LabelInfo>> {
    let mut parser = ZplParser::new();
    parser.parse(zpl_data).map_err(LabelizeError::Parse)
}

pub fn parse_epl(epl_data: &[u8]) -> error::Result<Vec<LabelInfo>> {
    EplParser::new().parse(epl_data).map_err(LabelizeError::Parse)
}

pub fn render_label_to_png(label: &LabelInfo, options: DrawerOptions) -> error::Result<Vec<u8>> {
    let renderer = Renderer::new();
    let mut png = Vec::new();
    renderer
        .draw_label_as_png(label, &mut png, options)
        .map_err(LabelizeError::Render)?;
    Ok(png)
}

pub fn zpl_to_png(zpl_data: &[u8], options: DrawerOptions) -> error::Result<Vec<u8>> {
    let labels = parse_zpl(zpl_data)?;
    let label = labels
        .into_iter()
        .next()
        .ok_or_else(|| LabelizeError::Parse("No labels found in input".to_string()))?;
    render_label_to_png(&label, options)
}

pub fn epl_to_png(epl_data: &[u8], options: DrawerOptions) -> error::Result<Vec<u8>> {
    let labels = parse_epl(epl_data)?;
    let label = labels
        .into_iter()
        .next()
        .ok_or_else(|| LabelizeError::Parse("No labels found in input".to_string()))?;
    render_label_to_png(&label, options)
}
