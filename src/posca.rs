use std::hash::Hash;

use palette::rgb::LinSrgb;

use crate::Pen;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait ColorPalette: Clone + Eq + PartialEq + Hash {
    fn available_colors() -> Vec<Self>;
    fn rgb_color(&self) -> LinSrgb;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter)]
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
        Self::iter().collect()
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
            Self::Orange => (0xf4, 0x67, 0x2c),
            Self::Red => (0xd3, 0x0a, 0x0a),
            Self::Pink => (0xfa, 0x54, 0xac),
            Self::Violet => (0x69, 0x3d, 0xae),
        };
        LinSrgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0).into_linear()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter)]
pub enum UniPosca24Pk {
    Black,
    White,
    Red,
    Pink,
    Blue,
    LightBlue,
    Green,
    LightGreen,
    SunshineYellow,
    Yellow,
    Orange,
    Brown,
    Violet,
    Silver,
    Grey,
    Gold,
    GlitterLightBlue,
    GlitterViolet,
    GlitterBlue,
    GlitterPink,
    GlitterRed,
    GlitterOrange,
    GlitterYellow,
}

impl ColorPalette for UniPosca24Pk {
    fn available_colors() -> Vec<Self> {
        Self::iter().collect()
    }

    fn rgb_color(&self) -> LinSrgb {
        let (r, g, b) = match self {
            Self::Black => (0x00, 0x00, 0x00),
            Self::White => (0xf6, 0xf7, 0xf9),
            Self::Red => (0xd3, 0x0a, 0x0a),
            Self::Pink => (0xfa, 0x54, 0xac),
            Self::Blue => (0x00, 0x00, 0xcb),
            Self::LightBlue => (0x51, 0xaf, 0xf7),
            Self::Green => (0x00, 0x7f, 0x2f),
            Self::LightGreen => (0x67, 0xcb, 0x57),
            Self::SunshineYellow => (0xfb, 0xfb, 0x76),
            Self::Yellow => (0xfd, 0xd0, 0x00),
            Self::Orange => (0xf4, 0x67, 0x2c),
            Self::Brown => (0x8c, 0x3a, 0x0a),
            Self::Violet => (0x69, 0x3d, 0xae),
            Self::Silver => (0xbe, 0xbf, 0xbd),
            Self::Grey => (0x73, 0x7e, 0x7f),
            Self::Gold => (0xc5, 0xa6, 0x43),
            Self::GlitterLightBlue => (0x47, 0x9b, 0xd8),
            Self::GlitterViolet => (0x9b, 0x7e, 0xc9),
            Self::GlitterBlue => (0x25, 0x72, 0xe8),
            Self::GlitterPink => (0xff, 0x97, 0xcf),
            Self::GlitterRed => (0xe1, 0x56, 0x72),
            Self::GlitterOrange => (0xee, 0x8c, 0x37),
            Self::GlitterYellow => (0xd8, 0xc9, 0x00),
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
        T::available_colors().into_iter().map(Self::new).collect()
    }

    fn nib_size_mm() -> f64 {
        0.7
    }

    fn rgb_color(&self) -> LinSrgb {
        self.color.rgb_color()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct UniPosca3C<T: ColorPalette> {
    pub color: T,
}

impl<T: ColorPalette> UniPosca3C<T> {
    pub fn new(color: T) -> UniPosca3C<T> {
        UniPosca3C { color }
    }
}

impl<T: ColorPalette> Pen for UniPosca3C<T> {
    fn available_colors() -> Vec<Self> {
        T::available_colors().into_iter().map(Self::new).collect()
    }

    fn nib_size_mm() -> f64 {
        1.3
    }

    fn rgb_color(&self) -> LinSrgb {
        self.color.rgb_color()
    }
}
