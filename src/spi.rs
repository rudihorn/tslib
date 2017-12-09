
#[allow(unused_imports)]
#[macro_use]
use common;

use core::any::Any;
use core::fmt::{Display, Formatter, Result};
use core::ops::Deref;
use core::ptr;
use core::marker::PhantomData;
use core::mem::transmute;

use gpio::{Input, PinOutput, GpioPin, Pin4, Pin5, Pin6, Pin7, Pin12, Pin13, Pin14, Pin15, PinMode, PinCnf1, PinCnf2};
use afio::{AfioSPIPeripheral, NotRemapped};
use blue_pill::stm32f103xx::{GPIOA, GPIOB, SPI1, SPI2, spi1, gpioa};


type_states!(IsConfigured, (NotConfigured, Configured));


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


pub enum SPIResult<T>
{
    Success(T),
    Error(SPIError)
}

#[derive(Copy, Clone)]
pub enum SPIError {
    /// No known error has occured
    None,
    NotReady,
    /// Timeout Failure
    /// 
    /// - SCL remained low for 25 ms
    /// - Master cumulative clock low extend time more than 10ms
    /// - Slave cumulative clock low extend time more than 25ms
    Timeout,
    /// Acknowledge Failure.
    /// 
    /// No acknowledge returned.
    AF,
    /// Arbitration Lost
    /// 
    /// Arbritation to the bus is lost to another master
    ARLO,
    /// Overrun / Underrun
    /// 
    /// - During reception a new byte is received even though the DR has not been read.
    /// - During transmission when a new byte should be sent, but the DR register has not been written to.
    OVR,
    /// Mode fault
    MODF,
    /// Bus Error
    BERR,
    /// CRC Error
    CRCERR,
}


impl SPIError {
}

impl Display for SPIError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            SPIError::None => write!(f, "None"),
            SPIError::Timeout => write!(f,"Timeout"),
            SPIError::AF => write!(f,"AF"),
            SPIError::ARLO => write!(f, "ARLO"),
            SPIError::OVR => write!(f, "OVR"),
            SPIError::BERR => write!(f, "BERR"),
            SPIError::MODF => write!(f, "MODF"),
            SPIError::CRCERR => write!(f, "CRCERR"),
            SPIError::NotReady => write!(f, "NotReady"),
        }
    }
}

pub struct SpiBusPorts<'a, S, P>(pub &'a S, PhantomData<P>)
where S: Any + SPI, P: IsConfigured;


impl<'a> SpiBusPorts<'a, SPI2, NotConfigured> {
    #[inline(always)]
    pub fn set_ports<M>(self, 
        pb12 : GpioPin<'a, GPIOB, Pin12, M, PinCnf2>, 
        pb13 : GpioPin<'a, GPIOB, Pin13, M, PinCnf2>,
        pb14 : GpioPin<'a, GPIOB, Pin14, Input, PinCnf1>,
        pb15 : GpioPin<'a, GPIOB, Pin15, M, PinCnf2>) 
        -> SpiBusPorts<'a, SPI2, Configured> where M : PinOutput + PinMode {
            unsafe {
                transmute(self)
            }
        }
}

impl<'a> SpiBusPorts<'a, SPI1, NotConfigured> {
    #[inline(always)]
    pub fn set_ports_normal<M>(self, 
        pa4 : GpioPin<'a, GPIOA, Pin4, M, PinCnf2>, 
        pa5 : GpioPin<'a, GPIOA, Pin5, M, PinCnf2>,
        pa6 : GpioPin<'a, GPIOA, Pin6, Input, PinCnf1>,
        pa7 : GpioPin<'a, GPIOA, Pin7, M, PinCnf2>,
        afio_spi : AfioSPIPeripheral<'a, SPI1, NotRemapped>) 
        -> SpiBusPorts<'a, SPI1, Configured> where M : PinOutput + PinMode {
            unsafe {
                transmute(self)
            }
        }

    /* #[inline(always)]
    pub fn set_ports_remapped<M>(self, 
        pb8 : GpioPin<'a, GPIOB, Pin8, M, PinCnf3>, 
        pb9 : GpioPin<'a, GPIOB, Pin9, M, PinCnf3>,
        afio_spi : AfioSPIPeripheral<'a, SPI1, Remapped>) 
        -> SpiBusPorts<'a, SPI1, Configured> where M : PinOutput + PinMode {
            unsafe {
                transmute(self)
            }
        } */
}

pub struct Spi<'a, S>(pub &'a S)
where S: Any + SPI;

impl<'a, S> Spi<'a, S>
where S: Any + SPI,
{
    pub fn start_init(&self) -> (
        SpiBusPorts<S, NotConfigured>
    ) {
        let spi = self.0;
        (
            SpiBusPorts(spi, PhantomData)
        )
    }

    pub fn complete_init(&self,
        bus_ports : SpiBusPorts<'a, S, Configured>
        ) {
        let spi = self.0;

        unsafe{ 
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
        }
    }

    pub fn listen(&self) {
        let spi = self.0;

        unsafe { 
            spi.cr2.modify(|_,w| { w.txeie().set_bit().rxneie().set_bit().errie().set_bit() });
        }
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
    
    pub fn read(&self) -> SPIResult<u8> {
        let spi1 = self.0;
        let sr = spi1.sr.read();


        if sr.ovr().bit_is_set() {
            SPIResult::Error(SPIError::OVR)
        } else if sr.modf().bit_is_set() {
            SPIResult::Error(SPIError::MODF)
        } else if sr.crcerr().bit_is_set() {
            SPIResult::Error(SPIError::CRCERR)
        } else if sr.rxne().bit_is_set() {
            // NOTE(write_volatile) see note above
            unsafe {
                SPIResult::Success(ptr::read_volatile(&spi1.dr as *const _ as *const u8))
            }
        } else {
            SPIResult::Error(SPIError::NotReady)
        }
    }
    
    pub fn send(&self, byte: u8) -> SPIError {
        let spi1 = self.0;
        let sr = spi1.sr.read();

        if sr.ovr().bit_is_set() {
            SPIError::OVR 
        } else if sr.modf().bit_is_set() {
            SPIError::MODF
        } else if sr.crcerr().bit_is_set() {
            SPIError::CRCERR
        } else if sr.txe().bit_is_set() {
            // NOTE(write_volatile) see note above
            unsafe {
                ptr::write_volatile(&spi1.dr as *const _ as *mut u8, byte)
            }
            SPIError::None
        } else {
            SPIError::NotReady
        }
    }
}