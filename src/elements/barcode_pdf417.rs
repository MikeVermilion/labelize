use super::field_orientation::FieldOrientation;
use super::reverse_print::ReversePrint;
use super::label_position::LabelPosition;

#[derive(Clone, Debug)]
pub struct BarcodePdf417 {
    pub orientation: FieldOrientation,
    pub row_height: i32,
    pub security: i32,
    pub columns: i32,
    pub rows: i32,
    pub truncate: bool,
}

#[derive(Clone, Debug)]
pub struct BarcodePdf417WithData {
    pub reverse_print: ReversePrint,
    pub barcode: BarcodePdf417,
    pub position: LabelPosition,
    pub data: String,
}
