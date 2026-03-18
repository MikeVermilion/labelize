use super::reverse_print::ReversePrint;
use super::label_position::LabelPosition;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphicFieldFormat {
    Hex = 1,
    Raw = 2,
    AR = 3,
}

#[derive(Clone, Debug)]
pub struct GraphicField {
    pub reverse_print: ReversePrint,
    pub position: LabelPosition,
    pub format: GraphicFieldFormat,
    pub data_bytes: i32,
    pub total_bytes: i32,
    pub row_bytes: i32,
    pub data: Vec<u8>,
    pub magnification_x: i32,
    pub magnification_y: i32,
}
