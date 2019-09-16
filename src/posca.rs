use std::hash::Hash;

use palette::rgb::LinSrgb;

use crate::Pen;

pub trait ColorPalette: Clone + Eq + PartialEq + Hash {
    fn available_colors() -> Vec<Self>;
    fn rgb_color(&self) -> LinSrgb;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UniPosca12Pk {
    Black,
    White,
    Brown,
    Blue,
    LightBlue,
    Green,
    LightGreen,
    SunshineYellow,
    Orange,
    Red,
    Pink,
    Violet,
}

impl ColorPalette for UniPosca12Pk {
    fn available_colors() -> Vec<Self> {
        vec![
            Self::Black,
            Self::White,
            Self::Brown,
            Self::Blue,
            Self::LightBlue,
            Self::Green,
            Self::LightGreen,
            Self::SunshineYellow,
            Self::Orange,
            Self::Red,
            Self::Pink,
            Self::Violet,
        ]
    }

    fn rgb_color(&self) -> LinSrgb {
        let (r, g, b) = match self {
            Self::Black => (0x00, 0x00, 0x00),
            Self::White => (0xf6, 0xf7, 0xf9),
            Self::Brown => (0x8c, 0x3a, 0x0a),
            Self::Blue => (0x00, 0x00, 0xcb),
            Self::LightBlue => (0x51, 0xaf, 0xf7),
            Self::Green => (0x00, 0x7f, 0x2f),
            Self::LightGreen => (0x67, 0xcb, 0x57),
            Self::SunshineYellow => (0xfb, 0xfb, 0x76),
            Self::Orange => (0xf64, 0x67, 0x2c),
            Self::Red => (0xd3, 0x0a, 0x0a),
            Self::Pink => (0xfa, 0x54, 0xac),
            Self::Violet => (0x69, 0x3d, 0xae),
        };
        LinSrgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0).into_linear()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct UniPosca1MC<T: ColorPalette> {
    pub color: T,
}

impl<T: ColorPalette> UniPosca1MC<T> {
    pub fn new(color: T) -> UniPosca1MC<T> {
        UniPosca1MC { color }
    }
}

impl<T: ColorPalette> Pen for UniPosca1MC<T> {
    fn available_colors() -> Vec<Self> {
        T::available_colors().into_iter()
            .map(Self::new)
            .collect()
    }

    fn nib_size_mm() -> f64 {
        0.7
    }

    fn rgb_color(&self) -> LinSrgb {
        self.color.rgb_color()
    }
}