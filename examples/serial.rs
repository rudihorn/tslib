#![no_std]
#![feature(proc_macro)]

pub extern crate tslib;
pub extern crate cortex_m;
pub extern crate cortex_m_rtfm as rtfm;
pub extern crate panic_abort;

pub use tslib::stm32f103xx as stm32;

use tslib::{Rcc, Afio, Usart, Gpio, Flash};
use tslib::prelude::*;

fn main() {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = Flash::new(dp.FLASH);

    let rcc = Rcc::new(dp.RCC);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    
    let afio = Afio::new(&dp.AFIO);
    let afio_periph = afio.get_peripherals();

    let gpiob = Gpio::new(dp.GPIOB);
    let pinsb = gpiob.get_pins(rcc.peripherals.iopb.enable());

    let pb6 = pinsb.6.set_output_10MHz().set_alt_output_push_pull();
    let pb7 = pinsb.7.set_input().set_floating_input();

    let _spi1 = Usart::new(
        dp.USART1, 
        Usart::ports_remapped(pb6, pb7, afio_periph.usart1.set_remapped()),
        9_600.bps(),
        clocks,
    );
}