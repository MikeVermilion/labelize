use super::reverse_print::ReversePrint;
use super::font::FontInfo;
use super::label_position::LabelPosition;
use super::field_alignment::FieldAlignment;
use super::field_block::FieldBlock;

#[derive(Clone, Debug)]
pub struct TextField {
    pub reverse_print: ReversePrint,
    pub font: FontInfo,
    pub position: LabelPosition,
    pub alignment: FieldAlignment,
    pub text: String,
    pub block: Option<FieldBlock>,
}
