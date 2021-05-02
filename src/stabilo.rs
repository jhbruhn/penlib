use std::hash::Hash;

use palette::rgb::LinSrgb;

use crate::Pen;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// TODO: There's more colours than these, these were the ones I have on hand
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter)]
pub enum StabiloColor {
    LightGreen = 43,
    Yellow = 44,
    Black = 46,
    Red = 40,
    Green = 36,
    Blue = 41,
    Purple = 55,
    Brown = 45,
    NeonPink = 56,
    Orange = 54,
}

impl From<StabiloColor> for LinSrgb {
    fn from(item: StabiloColor) -> LinSrgb {
        let color = match item {
            StabiloColor::LightGreen => (145, 193, 61),
            StabiloColor::Yellow => (250, 210, 0),
            StabiloColor::Black => (0, 19, 26),
            StabiloColor::Red => (227, 0, 15),
            StabiloColor::Green => (0, 146, 77),
            StabiloColor::Blue => (15, 66, 149),
            StabiloColor::Purple => (91, 34, 130),
            StabiloColor::Brown => (109, 64, 44),
            StabiloColor::NeonPink => (237, 109, 166),
            StabiloColor::Orange => (234, 87, 37),
        };

        LinSrgb::new(
            color.0 as f32 / 255.0,
            color.1 as f32 / 255.0,
            color.2 as f32 / 255.0,
        )
        .into_linear()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PointEightyEight {
    color: StabiloColor,
}

impl PointEightyEight {
    pub fn new(color: StabiloColor) -> Self {
        Self { color }
    }
}

impl Pen for PointEightyEight {
    fn available_colors() -> Vec<Self> {
        StabiloColor::iter()
            .map(|color| PointEightyEight::new(color))
            .collect()
    }

    fn nib_size_mm() -> f64 {
        0.4
    }

    fn rgb_color(&self) -> LinSrgb {
        self.color.into()
    }
}
