#![feature(get_type_id)]
#![feature(proc_macro)]
#![no_std]

#[macro_use]
pub mod common;

extern crate blue_pill;
#[allow(unused_imports)]
#[macro_use(iprint, iprintln)]
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

pub mod i2c;
pub mod rcc;
pub mod afio;
pub mod gpio;
pub mod spi;


