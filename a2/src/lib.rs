pub mod complex;
pub mod envelope;
pub mod wavetable;

pub mod prelude {
    pub use crate::envelope::*;
    pub use crate::wavetable::*;
}