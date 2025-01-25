#![feature(isqrt)]
#![feature(generic_const_exprs)]
#![feature(inherent_associated_types)]
#![feature(if_let_guard)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![feature(random)]

#[macro_use]
pub mod complex;

#[macro_use]
pub mod static_;
#[macro_use]
pub mod dynamic;


pub mod misc;
pub mod emulator;