#![no_std]
#![no_main]
#![feature(proc_macro)]

#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate tslib;
extern crate stm32f103xx as stm32;
extern crate panic_abort;

#[macro_use(block)]
pub extern crate nb;

use cortex_m::asm;
use rt::ExceptionFrame;
use tslib::{Rcc, Afio, Usart, Gpio, Flash};
use tslib::prelude::*;

entry!(main);

fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = Flash::new(dp.FLASH);

    let rcc = Rcc::new(dp.RCC);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    
    let afio = Afio::new(&dp.AFIO, rcc.peripherals.afio.enable());
    let afio_periph = afio.get_peripherals();

    let gpiob = Gpio::new(dp.GPIOB);
    let pinsb = gpiob.get_pins(rcc.peripherals.iopb.enable());

    let pb6 = pinsb.6.set_output_50MHz().set_alt_output_push_pull();
    let pb7 = pinsb.7.set_input().set_floating_input();

    let remap = afio_periph.usart1.set_remapped();
    let serial = Usart::new(
        dp.USART1, 
        Usart::ports_remapped(pb6, pb7, remap),
        rcc.peripherals.usart1.enable(),
        9_600.bps(),
        clocks,
    );

    let (mut tx, mut rx) = serial.split();

    block!(tx.write(b'X')).ok();

    loop {
        let mut c = block!(rx.read()).unwrap();
        c = c + 1;
        block!(tx.write(c)).unwrap();
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    asm::bkpt();
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    asm::bkpt();
    panic!("Unhandled exception (IRQn = {})", irqn);
}