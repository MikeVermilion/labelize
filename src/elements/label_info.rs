use super::label_element::LabelElement;

#[derive(Clone, Debug)]
pub struct LabelInfo {
    pub print_width: i32,
    pub inverted: bool,
    pub elements: Vec<LabelElement>,
}
