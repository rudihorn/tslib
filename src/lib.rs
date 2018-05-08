#![no_std]
#![feature(get_type_id)]
#![feature(never_type)]

#[macro_use]
pub mod common;

pub extern crate cortex_m;
pub extern crate nb;
pub extern crate embedded_hal as hal;
pub extern crate stm32f103xx;

pub mod i2c;
pub mod rcc;
pub mod afio;
pub mod gpio;
pub mod spi;
pub mod usart;
pub mod time;
pub mod flash;