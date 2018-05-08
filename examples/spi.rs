#![no_std]
#![feature(proc_macro)]

pub extern crate tslib;
pub extern crate cortex_m;
pub extern crate cortex_m_rtfm as rtfm;
pub extern crate panic_abort;

pub use tslib::stm32f103xx as stm32;

use tslib::{flash, rcc, afio, spi, gpio};

use flash::FlashExt;
use rcc::{Rcc};
use afio::Afio;
use gpio::{Gpio};
use spi::{Spi};

fn main() {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();

    let rcc = Rcc::new(dp.RCC);
    let _clocks = rcc.cfgr.freeze(&mut flash.acr);
    
    let afio = Afio::new(&dp.AFIO);
    let afio_periph = afio.get_peripherals();

    let gpioa = Gpio::new(dp.GPIOA);
    let pinsa = gpioa.get_pins(rcc.peripherals.iopa.enable());

    let pa4 = pinsa.4.set_output_10MHz().set_alt_output_push_pull();
    let pa5 = pinsa.5.set_output_10MHz().set_alt_output_push_pull();
    let pa6 = pinsa.6.set_input().set_floating_input();
    let pa7 = pinsa.7.set_output_10MHz().set_alt_output_push_pull();

    let _spi1 = Spi::new(
        &dp.SPI1, 
        Spi::normal_ports(pa4, pa5, pa6, pa7, afio_periph.spi1.set_not_remapped())
    );
}