//! I2C Bus
//! 
//! Credits for much of this module go to: 
//!     https://github.com/ilya-epifanov/stm32f103xx-hal/blob/master/src/i2c.rs
//! 
//! # I2C1
//! 
//! - SCL = PB6 (remapped: PB8)
//! - SDA = PB7 (remapped: PB9)
//! 
//! # I2C2
//! 
//! - SCL = PB10
//! - SDA = PB11

#[allow(unused_imports)]
use common;

use core::result::Result;
use core::any::{Any};
use core::fmt;
use core::ops::Deref;
use core::marker::PhantomData;

use stm32f103xx::{GPIOB, I2C1, I2C2, i2c1};

#[macro_use(block)]
use nb;

use afio::{AfioPeripheral, IsRemapped, Remapped, NotRemapped};
use gpio::{GpioPin, Pin6, Pin7, Pin8, Pin9, PinCnf3, PinOutput, PinMode};
use rcc::Clocks;
use time::Hertz;
use hal::blocking::i2c as hal_i2c;


pub unsafe trait I2C: Deref<Target = i2c1::RegisterBlock> {
}

unsafe impl I2C for I2C1 {

}

unsafe impl I2C for I2C2 {
}


#[derive(Copy, Clone)]
pub enum Error {
    /// No known error has occured
    None,
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
    /// Bus Error
    BERR,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::None => write!(f, "None"),
            Error::Timeout => write!(f,"Timeout"),
            Error::AF => write!(f,"AF"),
            Error::ARLO => write!(f, "ARLO"),
            Error::OVR => write!(f, "OVR"),
            Error::BERR => write!(f, "BERR")
        }
    }
}

pub struct I2c<S, R> where S: Any + I2C, R: IsRemapped {
    i2c: S,
    remapped: PhantomData<R>
}

pub struct I2cBusPorts<S, R> where S: Any + I2C, R: IsRemapped {
    i2c: PhantomData<S>,
    remapped: PhantomData<R>
}

#[derive(PartialEq, Clone, Copy)]
pub enum I2cDutyCycle {
    Ratio1to1,
    Ratio16to9,
}


impl<S, R> I2c<S, R>
where
    S: Any + I2C,
    R: IsRemapped
{

    /// Initializes the given I2c module
    /// 
    /// # Arguments
    /// - `i2c`: The I2C module.
    /// - `freq`: The I2c bus frequency, typically either 100 kHz (slow mode) or 400 kHz (fast mode)    
    /// - `duty`: The duty cycle of the I2c module.
    pub fn new<F>(i2c: S, ports: I2cBusPorts<S, R>, freq: F, clocks: Clocks, duty: I2cDutyCycle) -> Self 
        where F: Into<Hertz> {

        let freq = freq.into().0;
        assert!(freq <= 400_000);

        let i2cclk = clocks.pclk1().0;
        let freqrange = i2cclk / 1000000;

        i2c.cr2.modify(|_, w| unsafe { w.freq().bits(freqrange as u8) });

        let ratio = i2cclk / freq - 4;
        if freq > 100_000 {
            // slow mode
            assert!(duty == I2cDutyCycle::Ratio1to1);
            i2c.trise.write(|w| unsafe { w.trise().bits((freqrange + 1) as u8) });
            let ccr = i2cclk / (freq * 2); 
            i2c.ccr.modify(|_, w| unsafe { w.f_s().clear_bit().ccr().bits(ccr as u16) })
        } else {
            // fast mode
            let ccr = match duty {
                I2cDutyCycle::Ratio1to1 => (i2cclk / (freq * 3)).max(1),
                I2cDutyCycle::Ratio16to9 => (i2cclk / (freq * 25)).max(1),
            };

            i2c.trise.write(|w| unsafe { w.trise().bits((freqrange * 300 / 1000 + 1) as u8) });
            i2c.ccr.modify(|_, w| unsafe { w.f_s().set_bit()
                .duty().bit(duty == I2cDutyCycle::Ratio16to9)
                .ccr().bits(ccr as u16) });
        }
        
        i2c.cr1.modify(|_, w| w.pe().set_bit());

        Self {
            i2c: i2c,
            remapped: ports.remapped
        }
    }

    /// Enable all I2C module interrupts.
    pub fn listen(&mut self) {
        self.i2c.cr2.modify(|_, w| {w.itevten().set_bit().iterren().set_bit()})
    }

    fn read_sr1(&self) -> Result<i2c1::sr1::R, Error> {
        let sr1 = self.i2c.sr1.read();

        if sr1.timeout().bit_is_set() {
            return Err(Error::Timeout)
        } else if sr1.af().bit_is_set() {
            return Err(Error::AF)
        } else if sr1.arlo().bit_is_set() {
            return Err(Error::ARLO)
        } else if sr1.ovr().bit_is_set() {
            return Err(Error::OVR)
        } else if sr1.berr().bit_is_set() {
            return Err(Error::BERR)
        }

        Ok(sr1)
    }

    fn read_sr2(&self) -> Result<i2c1::sr2::R, Error> {
        let sr2 = self.i2c.sr2.read();
        Ok(sr2)
    }

    /// Try to generate a start signal. Complete using the
    /// `start_complete` function.
    pub fn start(&mut self) -> Result<(), Error> {
        self.i2c.cr1.modify(|_,w| w.start().set_bit());
        Ok(())
    }

    /// Give up control over the I2C bus. 
    pub fn stop(&mut self) -> Result<(), Error> {
        self.i2c.cr1.modify(|_,w| w.stop().set_bit());
        Ok(())
    }

    /// Returns `Ok(())` once the I2C module has finished transmitting
    /// the start signal, otherwise returns `Err(nb::Error::WouldBlock)`.
    /// 
    /// # Remarks
    /// 
    /// Use `block!(i2c.start_complete()).unwrap()` to continue polling
    /// until the start signal is generated.
    pub fn start_complete(&self) -> nb::Result<(), Error> {
        match self.read_sr1() {
            Ok(sr1) => if sr1.sb().bit_is_set() {Ok(())} else { Err(nb::Error::WouldBlock) },
            Err(err) => Err(nb::Error::Other(err)),
        }
    }

    /// Returns `Ok(())` if address has finished sending (master) or if 
    /// matching address has been received (slave). Returns `Err(nb::Error::WouldBlock)`
    /// otherwise.
    /// 
    /// # Remarks
    /// 
    /// Use `block!(i2c.addr_complete()).unwrap()` to continue polling
    /// until event happens.
    pub fn addr_complete(&self) -> nb::Result<(), Error> {
        match self.read_sr1() {
            Ok(sr1) => if sr1.addr().bit_is_set() {Ok(())} else { Err(nb::Error::WouldBlock) },
            Err(err) => Err(nb::Error::Other(err)),
        }
    }

    /// Returns `Ok(())` if the transmit buffer is empty. Returns `Err(nb::Error::WouldBlock)`
    /// otherwise.
    /// 
    /// # Remarks
    /// 
    /// Use `block!(i2c.transmit_empty()).unwrap()` to continue polling
    /// until event happens.
    pub fn transmit_empty(&self) -> nb::Result<(), Error> {
        match self.read_sr1() {
            Ok(sr1) => if sr1.tx_e().bit_is_set() { Ok(()) } else { Err(nb::Error::WouldBlock) },
            Err(err) => Err(nb::Error::Other(err))
        }
    }

    /// Returns `Ok(())` if the receive buffer is not empty. Returns `Err(nb::Error::WouldBlock)`
    /// otherwise.
    /// 
    /// # Remarks
    /// 
    /// Use `block!(i2c.receive_not_empty()).unwrap()` to continue polling
    /// until event happens.
    pub fn receive_not_empty(&self) -> nb::Result<(), Error> {
        match self.read_sr1() {
            Ok(sr1) => if sr1.rx_ne().bit_is_set() { Ok(()) } else { Err(nb::Error::WouldBlock) },
            Err(err) => Err(nb::Error::Other(err))
        }
    }

    /// Writes to the I2C data register. This function should most likely not be 
    /// used directly.
    pub unsafe fn write_unchecked(&mut self, dat: u8) -> Result<(), Error> {
        self.i2c.dr.write(|w| { w.dr().bits(dat) });

        Ok(())
    }

    /// Reads from the I2C data register. This should not be used directly.
    pub unsafe fn read_unchecked(&mut self) -> Result<u8, Error> {
        let dr = self.i2c.dr.read().dr().bits();
        Ok(dr)
    }
}

impl<S, R> hal_i2c::Write for I2c<S, R> 
    where S: I2C + Any, R: IsRemapped {

    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        assert!(bytes.len() > 0);

        self.start()?;
        block!(self.start_complete())?;

        unsafe { self.write_unchecked(addr & 0b1111_1110)? };
        block!(self.addr_complete())?;
        self.read_sr2()?;
        
        for b in bytes {
            block!(self.transmit_empty())?;
            unsafe { self.write_unchecked(*b)? };
        }
        block!(self.transmit_empty())?;

        self.stop()?;

        Ok(())
    }
}

impl<S, R> hal_i2c::WriteRead for I2c<S,R>
    where S: I2C + Any, R: IsRemapped {

    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        self.start()?;
        block!(self.start_complete())?;

        unsafe { self.write_unchecked(addr & 0b1111_1110)? };
        block!(self.addr_complete())?;
        self.read_sr2()?;

        for b in bytes {
            block!(self.transmit_empty())?;
            unsafe { self.write_unchecked(*b)? };
        }
        block!(self.transmit_empty())?;

        self.start()?;
        block!(self.start_complete())?;
        self.read_sr2()?;
        
        for b in buffer {
            block!(self.receive_not_empty())?;
            *b = unsafe { self.read_unchecked()? };
        }

        self.stop()?;

        Ok(())
    }
}

impl I2c<I2C1, NotRemapped> {
    #[inline(always)]
    pub fn ports_normal<M>( 
        _pb6 : GpioPin<GPIOB, Pin6, M, PinCnf3>, 
        _pb7 : GpioPin<GPIOB, Pin7, M, PinCnf3>,
        _afio_i2c : AfioPeripheral<I2C1, NotRemapped>) 
        -> I2cBusPorts<I2C1, NotRemapped> where M : PinOutput + PinMode {
            I2cBusPorts {
                i2c: PhantomData,
                remapped: PhantomData,
            }
        }
}

impl I2c<I2C1, Remapped> {
    #[inline(always)]
    pub fn ports_remapped<M>( 
        _pb8 : GpioPin<GPIOB, Pin8, M, PinCnf3>, 
        _pb9 : GpioPin<GPIOB, Pin9, M, PinCnf3>,
        _afio_i2c : AfioPeripheral<I2C1, Remapped>) 
        -> I2cBusPorts<I2C1, Remapped> where M : PinOutput + PinMode {
            I2cBusPorts {
                i2c: PhantomData,
                remapped: PhantomData
            }
        }
}

