//! A collection of characteristics for various pen brands and products.
//! This library was created to assist in creating SVG images for use by pen
//! plotters. Pen sizes and colors can be used to more carefully plan spacing
//! between paths, or to produce SVG images which will more closely resemble
//! the result of plotting to real paper.
pub mod blend;
pub mod papermate;
pub mod posca;
pub mod stabilo;
pub mod pilot;

use std::hash::Hash;

use palette::rgb::LinSrgb;
use palette::{Hsl, Lab};

pub trait Pen: Clone + Eq + PartialEq + Hash {
    fn available_colors() -> Vec<Self>;

    fn nib_size_mm() -> f64;

    fn rgb_color(&self) -> LinSrgb;

    fn hsl_color(&self) -> Hsl {
        self.rgb_color().into()
    }

    fn lab_color(&self) -> Lab {
        self.rgb_color().into()
    }

    fn closest_pen_to_color(icolor: Lab) -> Self {
        let available_pens = Self::available_colors();

        let (l, a, b) = icolor.into_components();

        let mut closest = available_pens[0].clone();
        let mut mindist = std::f32::MAX;

        for pen in available_pens {
            let (pl, pa, pb) = pen.lab_color().into_components();

            let dist = ((pl - l).powi(2) + (pa - a).powi(2) + (pb - b).powi(2)).sqrt();

            if dist < mindist {
                mindist = dist;
                closest = pen.clone();
            }
        }
        closest
    }

    fn rgb_pixel(&self) -> (u8, u8, u8) {
        let (r, g, b) = self.rgb_color().into_components();
        ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}
