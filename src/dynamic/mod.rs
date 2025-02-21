// Storing different size vectors and matrices as different types makes it nearly impossible to dynamically construct operators during emulation.

/*
TODO: Make get methods falible
*/

#[macro_use]
mod vector;
#[macro_use]
mod matrix;

mod state;
mod gate;

pub use state::*;
pub use gate::*;
pub use matrix::*;
pub use vector::*;