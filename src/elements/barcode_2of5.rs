use super::field_orientation::FieldOrientation;
use super::reverse_print::ReversePrint;
use super::label_position::LabelPosition;

#[derive(Clone, Debug)]
pub struct Barcode2of5 {
    pub orientation: FieldOrientation,
    pub height: i32,
    pub line: bool,
    pub line_above: bool,
    pub check_digit: bool,
}

#[derive(Clone, Debug)]
pub struct Barcode2of5WithData {
    pub reverse_print: ReversePrint,
    pub barcode: Barcode2of5,
    pub width: i32,
    pub width_ratio: f64,
    pub position: LabelPosition,
    pub data: String,
}
