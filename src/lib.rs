#![no_std]
#![feature(get_type_id)]

#[macro_use]
pub mod common;

extern crate cortex_m;
extern crate stm32f103xx;

pub mod i2c;
pub mod rcc;
pub mod afio;
pub mod gpio;
pub mod spi;


