use super::field_orientation::FieldOrientation;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FontInfo {
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub orientation: FieldOrientation,
}

impl Default for FontInfo {
    fn default() -> Self {
        FontInfo {
            name: "A".to_string(),
            width: 0.0,
            height: 0.0,
            orientation: FieldOrientation::Normal,
        }
    }
}

fn bitmap_font_sizes() -> &'static HashMap<&'static str, [f64; 2]> {
    use std::sync::OnceLock;
    static SIZES: OnceLock<HashMap<&str, [f64; 2]>> = OnceLock::new();
    SIZES.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("A", [9.0, 5.0]);
        m.insert("B", [11.0, 7.0]);
        m.insert("C", [18.0, 10.0]);
        m.insert("D", [18.0, 10.0]);
        m.insert("E", [28.0, 15.0]);
        m.insert("F", [26.0, 13.0]);
        m.insert("G", [60.0, 40.0]);
        m.insert("H", [21.0, 13.0]);
        m.insert("GS", [24.0, 24.0]);
        m
    })
}

impl FontInfo {
    pub fn get_size(&self) -> f64 {
        self.height
    }

    pub fn get_scale_x(&self) -> f64 {
        if self.height != 0.0 {
            self.get_width_to_height_ratio() * self.width / self.height
        } else {
            1.0
        }
    }

    pub fn is_standard_font(&self) -> bool {
        self.name == "0" || bitmap_font_sizes().contains_key(self.name.as_str())
    }

    pub fn with_adjusted_sizes(&self) -> FontInfo {
        let mut font = self.clone();
        let sizes = bitmap_font_sizes();

        if let Some(org_size) = sizes.get(font.name.as_str()) {
            // Bitmap font
            if font.width == 0.0 && font.height == 0.0 {
                font.width = org_size[1];
                font.height = org_size[0];
                return font;
            }

            if font.width == 0.0 {
                font.width = org_size[1] * (font.height / org_size[0]).round().max(1.0);
            } else {
                font.width = org_size[1] * (font.width / org_size[1]).round().max(1.0);
            }

            if font.height == 0.0 {
                font.height = org_size[0] * (font.width / org_size[1]).round().max(1.0);
            } else {
                font.height = org_size[0] * (font.height / org_size[0]).round().max(1.0);
            }

            font
        } else {
            // Scalable font (font 0)
            if font.width == 0.0 {
                font.width = font.height;
            }
            if font.height == 0.0 {
                font.height = font.width;
            }
            font.width = font.width.max(10.0);
            font.height = font.height.max(10.0);
            font
        }
    }

    fn get_width_to_height_ratio(&self) -> f64 {
        if self.name == "0" || self.name == "GS" {
            1.0
        } else {
            2.0
        }
    }
}
