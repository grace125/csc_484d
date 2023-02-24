use plotters::{
    element::ComposedElement,
    prelude::{Circle, DrawingBackend, EmptyElement, Text},
    style::{IntoFont, RGBColor, ShapeStyle},
};
use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

#[derive(Clone, Copy, Default)]
pub struct Complex {
    pub real: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(real: f32, im: f32) -> Self {
        Complex { real, im }
    }

    pub fn magnitude(&self) -> f32 {
        (self.real * self.real + self.im * self.im).sqrt()
    }

    pub fn as_labelled_point<B: DrawingBackend>(
        &self,
        c: RGBColor,
        label: impl Into<String>,
    ) -> ComposedElement<(f32, f32), B, Circle<(i32, i32), i32>, Text<(i32, i32), String>> {
        EmptyElement::at((self.real, self.im))
            + Circle::new((0, 0), 3, ShapeStyle::from(c).filled())
            + Text::new(label.into(), (10, 0), ("sans-serif", 15.0).into_font())
    }

    pub fn from_polar(r: f32, theta: f32) -> Complex {
        Complex {
            real: r * theta.cos(),
            im: r * theta.sin(),
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real * rhs.real - self.im * rhs.im,
            im: self.real * rhs.im + self.im * rhs.real,
        }
    }
}

impl Add<Complex> for Complex {
    type Output = Self;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            im: self.im + rhs.im,
        }
    }
}

impl Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}j", self.real, self.im)
    }
}

// impl<DB: DrawingBackend> Drawable<DB> for Complex {
//     fn draw<I: Iterator<Item = BackendCoordOnly>>(
//         &self,
//         mut pos: I,
//         backend: DB,
//         _: (u32, u32)
//     ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
//         Ok(())
//     }
// }
