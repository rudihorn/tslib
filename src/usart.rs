
#[allow(unused_imports)]
use common;

use core::any::Any;
use core::ops::Deref;
use core::ptr;
use core::marker::PhantomData;

use hal;
use nb;
use rcc;

use rcc::{Clocks, RccPeripheral};
use time::Bps;
use gpio::{Input, PinOutput, GpioPin, Pin6, Pin7, Pin9, Pin10, PinMode, PinCnf1, PinCnf2};
use afio::{AfioPeripheral, IsRemapped, Remapped, NotRemapped};
use stm32f103xx::{GPIOA, GPIOB, USART1, USART2, USART3, usart1};

type_states!(IsConfigured, (NotConfigured, Configured));

/// SPI instance that can be used with the `Spi` abstraction
pub unsafe trait USART: Deref<Target = usart1::RegisterBlock> {
    fn ptr() -> *const usart1::RegisterBlock;
}

unsafe impl USART for USART1 {
    fn ptr() -> *const usart1::RegisterBlock { USART1::ptr() }
}

unsafe impl USART for USART2 {
    fn ptr() -> *const usart1::RegisterBlock { USART2::ptr() }
}

unsafe impl USART for USART3 {
    fn ptr() -> *const usart1::RegisterBlock { USART3::ptr() }
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

pub struct UsartBusPorts<U, R> where U: Any + USART, R: IsRemapped {
    usart: PhantomData<U>, 
    remapped: PhantomData<R>,
}

pub struct UsartTx<U, R> where U: Any + USART, R: IsRemapped {
    usart: PhantomData<U>, 
    remapped: PhantomData<R>,
}
pub struct UsartRx<U, R> where U: Any + USART, R: IsRemapped {
    usart: PhantomData<U>, 
    remapped: PhantomData<R>,
}

pub struct Usart<U, R> where U : Any+USART, R: IsRemapped {
    usart: U,
    remapped: PhantomData<R>,
}

impl<U, R> Usart<U, R> where U : Any+USART, R: IsRemapped {
    pub fn new(usart: U, ports : UsartBusPorts<U, R>, _rcc_periph: RccPeripheral<U, rcc::Enabled>, baud_rate: Bps, clocks: Clocks) -> Self {

        usart.cr3.write(|w| w.dmat().set_bit()
            .dmar().set_bit());

        let brr = clocks.pclk2().0 / baud_rate.0;
        assert!(brr > 16, "impossible baud rate");
        usart.brr.write(|w| unsafe { w.bits(brr) });

        // uart enable, receiver enable, transmitter enable
        usart.cr1.write(|w| w.ue().set_bit()
            .re().set_bit().te().set_bit());

        Self {
            usart: usart,
            remapped: ports.remapped
        }
    }

    pub fn listen(&mut self, event: Event) {
        match event {
            Event::Rxne => self.usart.cr1.modify(|_, w| w.rxneie().set_bit()),
            Event::Txe => self.usart.cr1.modify(|_, w| w.txeie().set_bit()),
        }
    }

    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::Rxne => self.usart.cr1.modify(|_, w| w.rxneie().clear_bit()),
            Event::Txe => self.usart.cr1.modify(|_, w| w.txeie().clear_bit()),
        }
    }

    pub fn split(self) -> (UsartTx<U, R>, UsartRx<U, R>) {
        (UsartTx { usart: PhantomData, remapped: self.remapped }, UsartRx{ usart: PhantomData, remapped: self.remapped })
    }
}

impl<U, R> hal::serial::Read<u8> for UsartRx<U, R>
    where U: USART + Any, R: IsRemapped {

    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*U::ptr()).sr.read() };
        
        Err(if sr.pe().bit_is_set() {
            nb::Error::Other(Error::Parity)
        } else if sr.fe().bit_is_set() {
            nb::Error::Other(Error::Framing)
        } else if sr.ne().bit_is_set() {
            nb::Error::Other(Error::Noise)
        } else if sr.ore().bit_is_set() {
            nb::Error::Other(Error::Overrun)
        } else if sr.rxne().bit_is_set() {
            // NOTE(read_volatile) see `write_volatile` below
            return Ok(unsafe {
                ptr::read_volatile(&(*U::ptr()).dr as *const _ as *const _)
            });
        } else {
            nb::Error::WouldBlock
        })
    }
}

impl<U, R> hal::serial::Write<u8> for UsartTx<U, R>
    where U: USART + Any, R: IsRemapped {
        type Error = !;

        fn flush(&mut self) -> nb::Result<(), !> {
            let sr = unsafe { (*U::ptr()).sr.read() };

            if sr.txe().bit_is_set() {
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        }

        fn write(&mut self, byte: u8) -> nb::Result<(), !> {
            let sr = unsafe { (*U::ptr()).sr.read() };

            if sr.txe().bit_is_set() {
                unsafe {
                    ptr::write_volatile(&(*U::ptr()).dr as *const _ as *mut _, byte)
                }
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        }
    }

impl Usart<USART1, NotRemapped> {
    #[inline(always)]
    pub fn ports_normal<M>( 
        _pa9_tx : GpioPin<GPIOA, Pin9, M, PinCnf2>, 
        _pa10_rx : GpioPin<GPIOA, Pin10, Input, PinCnf1>,
        _afio : AfioPeripheral<USART1, NotRemapped>) 
        -> UsartBusPorts<USART1, NotRemapped> where M : PinOutput + PinMode {
            UsartBusPorts {
                usart: PhantomData,
                remapped: PhantomData
            }
        }
}

impl Usart<USART1, Remapped> {
    #[inline(always)]
    pub fn ports_remapped<M>( 
        _pb6_tx : GpioPin<GPIOB, Pin6, M, PinCnf2>, 
        _pb7_rx : GpioPin<GPIOB, Pin7, Input, PinCnf1>,
        _afio : AfioPeripheral<USART1, Remapped>) 
        -> UsartBusPorts<USART1, Remapped> where M : PinOutput + PinMode {
            UsartBusPorts {
                usart: PhantomData,
                remapped: PhantomData
            }
        }
}