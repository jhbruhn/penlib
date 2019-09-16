use std::hash::Hash;

use palette::rgb::LinSrgb;

use crate::Pen;

pub trait InkJoyGelPenSize: Clone + Eq + PartialEq + Hash {
    fn nib_size() -> f64;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PointSevenMM;
impl InkJoyGelPenSize for PointSevenMM {
    fn nib_size() -> f64 {
        0.7
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PointFiveMM;
impl InkJoyGelPenSize for PointFiveMM {
    fn nib_size() -> f64 {
        0.5
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum InkJoyGelPenColor {
    Red,
    Berry,
    Pink,
    Orange,
    Yellow,
    Green,
    Lime,
    SlateBlue,
    Blue,
    BrightBlue,
    Teal,
    Purple,
    Cocoa,
    Black,
}

impl InkJoyGelPenColor {
    fn rgb_color(&self) -> LinSrgb {
        let (r, g, b) = match self {
            InkJoyGelPenColor::Red => (0xd1, 0x24, 0x31),
            InkJoyGelPenColor::Berry => (0xc1, 0x52, 0x9e),
            InkJoyGelPenColor::Pink => (0xd8, 0x40, 0x8c),
            InkJoyGelPenColor::Orange => (0xf3, 0x6c, 0x38),
            InkJoyGelPenColor::Yellow => (0xff, 0xda, 0x3a),
            InkJoyGelPenColor::Green => (0x00, 0xa8, 0x5d),
            InkJoyGelPenColor::Lime => (0xa6, 0xd0, 0x60),
            InkJoyGelPenColor::SlateBlue => (0x28, 0x62, 0x8f),
            InkJoyGelPenColor::Blue => (0x0a, 0x2e, 0x83),
            InkJoyGelPenColor::BrightBlue => (0x47, 0xb7, 0xe6),
            InkJoyGelPenColor::Teal => (0x00, 0x9b, 0xa8),
            InkJoyGelPenColor::Purple => (0x78, 0x5b, 0xa7),
            InkJoyGelPenColor::Cocoa => (0x8e, 0x61, 0x5e),
            InkJoyGelPenColor::Black => (0x37, 0x36, 0x3d),
        };
        LinSrgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0).into_linear()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct InkJoyGelPen<T: InkJoyGelPenSize> {
    color: InkJoyGelPenColor,
    _size: std::marker::PhantomData<T>,
}

impl<T: InkJoyGelPenSize> InkJoyGelPen<T> {
    pub fn new(color: InkJoyGelPenColor) -> Self {
        let _size = std::marker::PhantomData;
        InkJoyGelPen { color, _size }
    }
}

impl<T: InkJoyGelPenSize> Pen for InkJoyGelPen<T> {
    fn available_colors() -> Vec<Self> {
        vec![
            InkJoyGelPen::new(InkJoyGelPenColor::Red),
            InkJoyGelPen::new(InkJoyGelPenColor::Berry),
            InkJoyGelPen::new(InkJoyGelPenColor::Pink),
            InkJoyGelPen::new(InkJoyGelPenColor::Orange),
            InkJoyGelPen::new(InkJoyGelPenColor::Yellow),
            InkJoyGelPen::new(InkJoyGelPenColor::Green),
            InkJoyGelPen::new(InkJoyGelPenColor::Lime),
            InkJoyGelPen::new(InkJoyGelPenColor::SlateBlue),
            InkJoyGelPen::new(InkJoyGelPenColor::Blue),
            InkJoyGelPen::new(InkJoyGelPenColor::BrightBlue),
            InkJoyGelPen::new(InkJoyGelPenColor::Teal),
            InkJoyGelPen::new(InkJoyGelPenColor::Purple),
            InkJoyGelPen::new(InkJoyGelPenColor::Cocoa),
            InkJoyGelPen::new(InkJoyGelPenColor::Black),
        ]
    }

    fn nib_size_mm() -> f64 {
        T::nib_size()
    }

    fn rgb_color(&self) -> LinSrgb {
        self.color.rgb_color()
    }
}