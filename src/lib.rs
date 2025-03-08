#![feature(generic_const_exprs)]
#![allow(unused_mut)]
#![feature(random)]
#![allow(unused_assignments)]

#[macro_use]
pub mod complex;

#[macro_use]
pub mod static_;

#[macro_use]
pub mod dynamic;

pub mod misc;
pub mod emulator;