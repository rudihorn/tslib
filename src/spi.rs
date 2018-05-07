
#[allow(unused_imports)]
use common;

use core::any::Any;
use core::ops::Deref;
use core::ptr;
use core::marker::PhantomData;

use nb;
use hal;

use gpio::{Input, GpioPin, Pin4, Pin5, Pin6, Pin7, Pin12, Pin13, Pin14, Pin15, PinMode, PinCnf1, PinCnf2};
use afio::{AfioSPI1Peripheral, IsRemapped, Remapped, NotRemapped};
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

impl<'a, S, R> Spi<'a, S, R>
where S: Any + SPI, R: IsRemapped,
{
    pub fn new(spi: &'a S, _ports: SpiBusPorts<S, R>) -> Self {

        // enable slave select
        spi.cr2.modify(|_, w| { w.ssoe().set_bit() });

        spi.cr1.write(|w| {
            w.cpha()
            .set_bit()
            .cpol()
            .set_bit()
            .mstr()
            .set_bit()
            .br()
            .bits(0b10)
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

impl<'a> Spi<'a, SPI1, NotRemapped> {
    pub fn remapped_ports<M>(
        _pa4 : GpioPin<'a, GPIOA, Pin4, M, PinCnf2>, 
        _pa5 : GpioPin<'a, GPIOA, Pin5, M, PinCnf2>,
        _pa6 : GpioPin<'a, GPIOA, Pin6, Input, PinCnf1>,
        _pa7 : GpioPin<'a, GPIOA, Pin7, M, PinCnf2>,
        _afio_spi : AfioSPI1Peripheral<'a, NotRemapped>) -> SpiBusPorts<SPI1, NotRemapped> 
        where M : PinMode {
            SpiBusPorts(PhantomData)
        }
}

impl<'a> Spi<'a, SPI2, Remapped> {
    pub fn remapped_ports<M>(
        _pa4 : GpioPin<'a, GPIOA, Pin4, M, PinCnf2>, 
        _pb12 : GpioPin<'a, GPIOB, Pin12, M, PinCnf2>, 
        _pb13 : GpioPin<'a, GPIOB, Pin13, M, PinCnf2>,
        _pb14 : GpioPin<'a, GPIOB, Pin14, Input, PinCnf1>,
        _pb15 : GpioPin<'a, GPIOB, Pin15, M, PinCnf2>) -> SpiBusPorts<SPI2, NotRemapped> 
        where M : PinMode {
            SpiBusPorts(PhantomData)        
        }
}