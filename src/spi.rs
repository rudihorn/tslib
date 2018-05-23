//! Serial Peripheral Interface
//! 
//! Used for communicating with peripheral devices.
//! 
//! # Example
//! 
//! ```
//! let pa4 = pinsa.4.set_output_10MHz().set_alt_output_push_pull();
//! let pa5 = pinsa.5.set_output_10MHz().set_alt_output_push_pull();
//! let pa6 = pinsa.6.set_input().set_floating_input();
//! let pa7 = pinsa.7.set_output_10MHz().set_alt_output_push_pull();
//!
//! let _spi1 = Spi::new(
//!     &dp.SPI1, 
//!     Spi::ports_normal(pa4, pa5, pa6, pa7, afio_periph.spi1.set_not_remapped()),
//!     rcc.peripherals.sp1.enable(),
//! );
//! ```
//! 

#[allow(unused_imports)]
use common;

use core::any::Any;
use core::ops::Deref;
use core::ptr;
use core::marker::PhantomData;

use nb;
use hal;
use rcc;

use time::Hertz;
use rcc::{Clocks, RccPeripheral};
use gpio::{Input, GpioPin, Pin4, Pin5, Pin6, Pin7, Pin12, Pin13, Pin14, Pin15, PinMode, PinCnf1, PinCnf2};
use afio::{AfioPeripheral, IsRemapped, Remapped, NotRemapped};
use stm32f103xx::{GPIOA, GPIOB, SPI1, SPI2, spi1, gpioa};


/// SPI instance that can be used with the `Spi` abstraction
pub unsafe trait SPI: Deref<Target = spi1::RegisterBlock> {
    /// GPIO block associated to this SPI instance
    type GPIO: Deref<Target = gpioa::RegisterBlock>;
}

unsafe impl SPI for SPI1 {
    type GPIO = GPIOA;
}

unsafe impl SPI for SPI2 {
    type GPIO = GPIOB;
}

#[derive(Copy, Clone)]
pub enum Error {
    /// Timeout Failure
    /// 
    /// - SCL remained low for 25 ms
    /// - Master cumulative clock low extend time more than 10ms
    /// - Slave cumulative clock low extend time more than 25ms
    Timeout,
    /// Acknowledge Failure.
    /// 
    /// No acknowledge returned.
    AcknowledgementFailure,
    /// Arbitration Lost
    /// 
    /// Arbritation to the bus is lost to another master
    ArbitrationLost,
    /// Overrun / Underrun
    /// 
    /// - During reception a new byte is received even though the DR has not been read.
    /// - During transmission when a new byte should be sent, but the DR register has not been written to.
    Overrun,
    /// Mode fault
    ModeFault,
    /// Bus Error
    BusError,
    /// CRC Error
    CrcError,
}


pub struct SpiBusPorts<S, R>(PhantomData<(S, R)>)
where S: Any + SPI, R: IsRemapped;

pub struct Spi<'a, S, R>(pub &'a S, PhantomData<R>)
where S: Any + SPI, R: IsRemapped;

impl<'a> Spi<'a, SPI1, NotRemapped> {
    pub fn ports_normal<M>(
        _pa4 : GpioPin<GPIOA, Pin4, M, PinCnf2>, 
        _pa5 : GpioPin<GPIOA, Pin5, M, PinCnf2>,
        _pa6 : GpioPin<GPIOA, Pin6, Input, PinCnf1>,
        _pa7 : GpioPin<GPIOA, Pin7, M, PinCnf2>,
        _afio_spi : AfioPeripheral<SPI1, NotRemapped>) -> SpiBusPorts<SPI1, NotRemapped> 
        where M : PinMode {
            SpiBusPorts(PhantomData)
        }
}

impl<'a> Spi<'a, SPI2, Remapped> {
    pub fn ports_remapped<M>(
        _pa4 : GpioPin<GPIOA, Pin4, M, PinCnf2>, 
        _pb12 : GpioPin<GPIOB, Pin12, M, PinCnf2>, 
        _pb13 : GpioPin<GPIOB, Pin13, M, PinCnf2>,
        _pb14 : GpioPin<GPIOB, Pin14, Input, PinCnf1>,
        _pb15 : GpioPin<GPIOB, Pin15, M, PinCnf2>) -> SpiBusPorts<SPI2, NotRemapped> 
        where M : PinMode {
            SpiBusPorts(PhantomData)        
        }
}

impl<'a, S, R> Spi<'a, S, R>
where S: Any + SPI, R: IsRemapped,
{
    pub fn new(spi: &'a S, _ports: SpiBusPorts<S, R>, _rcc_periph: RccPeripheral<S, rcc::Enabled>, freq: Hertz, clocks: Clocks) -> Self {

        // enable slave select
        spi.cr2.modify(|_, w| { w.ssoe().set_bit() });

        let br = match clocks.pclk2().0 / freq.0 {
            0 => unreachable!(),
            1...2 => 0b000,
            3...5 => 0b001,
            6...11 => 0b010,
            12...23 => 0b011,
            24...47 => 0b100,
            48...95 => 0b101,
            96...191 => 0b110,
            _ => 0b111,
        };

        spi.cr1.write(|w| {
            w.cpha()
            .set_bit()
            .cpol()
            .set_bit()
            .mstr()
            .set_bit()
            .br()
            .bits(br)
            .lsbfirst()
            .clear_bit()
            .ssm()
            .clear_bit()
            .rxonly()
            .clear_bit()
            .dff()
            .clear_bit()
            .bidimode()
            .clear_bit()
        });

        Spi(spi, PhantomData)
    }

    pub fn listen(&self, tx : bool, rx : bool) {
        let spi = self.0;

        spi.cr2.modify(|_,w| { 
            let w = w.txeie();
            let w = if tx { w.set_bit() } else { w.clear_bit() };
            let w = w.rxneie();
            let w = if rx { w.set_bit() } else { w.clear_bit() }; 
            w.errie().set_bit() });
    }

    /// Disables the SPI bus
    ///
    /// **NOTE** This drives the NSS pin high
    pub fn disable(&self) {
        self.0.cr1.modify(|_, w| w.spe().clear_bit())
    }

    /// Enables the SPI bus
    ///
    /// **NOTE** This drives the NSS pin low
    pub fn enable(&self) {
        self.0.cr1.modify(|_, w| w.spe().set_bit())
    }
}

impl<'a, S, R> hal::spi::FullDuplex<u8> for Spi<'a, S, R>
    where S : Any + SPI, R : IsRemapped {
        type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        let sr = self.0.sr.read();

        Err(if sr.ovr().bit_is_set() {
            nb::Error::Other(Error::Overrun)
        } else if sr.modf().bit_is_set() {
            nb::Error::Other(Error::ModeFault)
        } else if sr.crcerr().bit_is_set() {
            nb::Error::Other(Error::CrcError)
        } else if sr.rxne().bit_is_set() {
            return Ok(unsafe { 
                ptr::read_volatile(&self.0.dr as *const _ as *const u8)
            });
        } else {
            nb::Error::WouldBlock
        })
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
        let sr = self.0.sr.read();

        Err(if sr.ovr().bit_is_set() {
            nb::Error::Other(Error::Overrun)
        } else if sr.modf().bit_is_set() {
            nb::Error::Other(Error::ModeFault)
        } else if sr.crcerr().bit_is_set() {
            nb::Error::Other(Error::CrcError)
        } else if sr.rxne().bit_is_set() {
            unsafe { ptr::write_volatile(&self.0.dr as * const _ as *mut u8, byte) }
            return Ok(());
        } else {
            nb::Error::WouldBlock
        })
    }
}
