#[derive(Clone, Debug, Default)]
pub struct LabelPosition {
    pub x: i32,
    pub y: i32,
    pub calculate_from_bottom: bool,
    pub automatic_position: bool,
}

impl LabelPosition {
    pub fn add(&self, other: &LabelPosition) -> LabelPosition {
        LabelPosition {
            x: self.x + other.x,
            y: self.y + other.y,
            calculate_from_bottom: self.calculate_from_bottom,
            automatic_position: self.automatic_position,
        }
    }
}
