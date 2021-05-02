use std::hash::Hash;

use palette::rgb::LinSrgb;

use crate::Pen;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// TODO: There's more colours than these, these were the ones I have on hand
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter)]
pub enum PilotColor {
    Black,
    Red,
    Green,
    Blue,
}

impl From<PilotColor> for LinSrgb {
    fn from(item: PilotColor) -> LinSrgb {
        let color = match item {
            PilotColor::Black => (0, 0, 0),
            PilotColor::Red => (226, 0, 26),
            PilotColor::Green => (0, 137, 70),
            PilotColor::Blue => (0, 65, 134),
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
pub struct HiTecpointV5 {
    color: PilotColor,
}

impl HiTecpointV5 {
    pub fn new(color: PilotColor) -> Self {
        Self { color }
    }
}

impl Pen for HiTecpointV5 {
    fn available_colors() -> Vec<Self> {
        PilotColor::iter()
            .map(|color| HiTecpointV5::new(color))
            .collect()
    }

    /// The size is in the name, 0.5mm, but the actual resulting lines have a size of around 0.3mm
    fn nib_size_mm() -> f64 {
        0.3
    }

    fn rgb_color(&self) -> LinSrgb {
        self.color.into()
    }
}
