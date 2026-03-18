use super::text_alignment::TextAlignment;

#[derive(Clone, Debug)]
pub struct FieldBlock {
    pub max_width: i32,
    pub max_lines: i32,
    pub line_spacing: i32,
    pub alignment: TextAlignment,
    pub hanging_indent: i32,
}

impl Default for FieldBlock {
    fn default() -> Self {
        FieldBlock {
            max_width: 0,
            max_lines: 1,
            line_spacing: 0,
            alignment: TextAlignment::Left,
            hanging_indent: 0,
        }
    }
}
