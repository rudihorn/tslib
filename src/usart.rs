
#[allow(unused_imports)]
use common;

use core::any::Any;
use core::fmt::{Display, Formatter, Result};
use core::ops::Deref;
use core::ptr;
use core::marker::PhantomData;
use core::mem::transmute;

use gpio::{Input, PinOutput, GpioPin, Pin6, Pin7, Pin9, Pin10, PinMode, PinCnf1, PinCnf3};
use afio::{AfioUSART1Peripheral, Remapped, NotRemapped};
use stm32f103xx::{GPIOA, GPIOB, USART1, USART2, usart1};

type_states!(IsConfigured, (NotConfigured, Configured));

/// SPI instance that can be used with the `Spi` abstraction
pub unsafe trait USART: Deref<Target = usart1::RegisterBlock> {
}

unsafe impl USART for USART1 {
}

unsafe impl USART for USART2 {
}

/// Interrupt event
pub enum Event {
    /// New data has been received
    Rxne,
    /// New data can be sent
    Txe,
}

/// Serial error
#[derive(Debug)]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
    #[doc(hidden)] _Extensible,
}

pub struct UsartBusPorts<S, P>(PhantomData<(S, P)>)
where S: Any + USART, P: IsConfigured;


pub struct Usart<U>(pub U) where U : Any+USART;

impl<U> Usart<U> where U : Any+USART {
    pub fn init(&mut self, _ports : UsartBusPorts<U, Configured>) {

        self.0.cr3.write(|w| w.dmat().set_bit()
            .dmar().set_bit());

        // setup BAUD rate of 115200
        // 8 MHz / 115.2 kHz = 69.444
        unsafe { self.0.brr.write(|w| { w.bits(69) }); }

        // uart enable, receiver enable, transmitter enable
        self.0.cr1.write(|w| w.ue().set_bit()
            .re().set_bit().te().set_bit());
    }

    pub fn listen(&mut self) {
        self.0.cr1.modify(|_, w| {w.rxneie().set_bit().txeie().set_bit()});
    }

    pub fn get_write_state(&mut self) {
        let state = self.0.sr.read();
    }
}

impl Usart<USART1> {
    #[inline(always)]
    pub fn set_ports_normal<M>(&self, 
        _pa9_tx : GpioPin<GPIOA, Pin9, M, PinCnf3>, 
        _pa10_rx : GpioPin<GPIOA, Pin10, Input, PinCnf1>,
        _afio : AfioUSART1Peripheral<NotRemapped>) 
        -> UsartBusPorts<USART1, Configured> where M : PinOutput + PinMode {
            UsartBusPorts(PhantomData)
        }

    #[inline(always)]
    pub fn set_ports_remapped<M>(&self, 
        _pb6_tx : GpioPin<GPIOB, Pin6, M, PinCnf3>, 
        _pb7_rx : GpioPin<GPIOB, Pin7, Input, PinCnf1>,
        _afio : AfioUSART1Peripheral<Remapped>) 
        -> UsartBusPorts<USART1, Configured> where M : PinOutput + PinMode {
            UsartBusPorts(PhantomData)
        }
}