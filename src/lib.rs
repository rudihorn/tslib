#![no_std]
#![feature(get_type_id)]

#[macro_use]
pub mod common;

#[macro_use(iprintln)]
pub extern crate cortex_m;
pub extern crate stm32f103xx_hal;

pub use stm32f103xx_hal::stm32f103xx;

pub mod i2c;
pub mod rcc;
pub mod afio;
pub mod gpio;
pub mod spi;


