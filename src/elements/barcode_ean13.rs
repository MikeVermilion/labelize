use super::field_orientation::FieldOrientation;
use super::reverse_print::ReversePrint;
use super::label_position::LabelPosition;

#[derive(Clone, Debug)]
pub struct BarcodeEan13 {
    pub orientation: FieldOrientation,
    pub height: i32,
    pub line: bool,
    pub line_above: bool,
}

#[derive(Clone, Debug)]
pub struct BarcodeEan13WithData {
    pub reverse_print: ReversePrint,
    pub barcode: BarcodeEan13,
    pub width: i32,
    pub position: LabelPosition,
    pub data: String,
}
