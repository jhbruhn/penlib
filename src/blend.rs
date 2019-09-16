use palette::Mix;
use palette::rgb::LinSrgb;

use crate::Pen;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PenBlend<T: Pen> {
    pub pen_a: T,
    pub pen_b: T,
}

impl<T: Pen> PenBlend<T> {
    pub fn new(pen_a: T, pen_b: T) -> Self {
        PenBlend { pen_a, pen_b }
    }
}

impl<T: Pen> Pen for PenBlend<T> {
    fn nib_size_mm() -> f64 {
        T::nib_size_mm()
    }

    fn available_colors() -> Vec<Self> {
        let mut results = vec![];

        for pen_a in T::available_colors() {
            for pen_b in T::available_colors() {
                results.push(Self::new(pen_a.clone(), pen_b.clone()));
            }
        }

        results
    }

    fn rgb_color(&self) -> LinSrgb {
        self.pen_a.rgb_color().mix(&self.pen_b.rgb_color(), 0.5)
    }
}