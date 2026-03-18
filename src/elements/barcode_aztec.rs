use super::field_orientation::FieldOrientation;
use super::reverse_print::ReversePrint;
use super::label_position::LabelPosition;

#[derive(Clone, Debug)]
pub struct BarcodeAztec {
    pub orientation: FieldOrientation,
    pub magnification: i32,
    pub size: i32,
}

#[derive(Clone, Debug)]
pub struct BarcodeAztecWithData {
    pub reverse_print: ReversePrint,
    pub barcode: BarcodeAztec,
    pub position: LabelPosition,
    pub data: String,
}
