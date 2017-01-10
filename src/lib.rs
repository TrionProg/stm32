//#![deny(warnings)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]
#![feature(const_fn)]
#![feature(more_struct_aliases)]
#![feature(alloc)]

extern crate volatile_register;

pub mod register;
pub mod rcc;

#[macro_use]
pub mod gpio;

pub mod timer;

extern crate alloc;

pub use alloc::boxed::Box;
