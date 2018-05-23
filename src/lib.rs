//! An experimental HAL for the STM32f103xx microcontrollers.
//! 
//! # Examples
//! 
//! See the [examples] module.
//! 
//! [examples]: examples/index.html

#![no_std]
#![feature(get_type_id)]
#![feature(never_type)]

#[macro_use]
pub mod common;

pub extern crate cortex_m;

#[macro_use(block)]
pub extern crate nb;

pub extern crate embedded_hal as hal;
pub extern crate stm32f103xx;

#[cfg(feature = "doc")]
pub mod examples;

pub mod prelude;

pub mod i2c;
pub mod rcc;
pub mod afio;
pub mod gpio;
pub mod spi;
pub mod usart;
pub mod time;
pub mod flash;

pub use flash::Flash;
pub use usart::Usart;
pub use spi::Spi;
pub use rcc::Rcc;
pub use afio::Afio;
pub use gpio::{Gpio, GpioPin};