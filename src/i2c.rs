//! I2C Bus
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

use core::any::{Any, TypeId};
use core::fmt::{Display, Formatter, Result};
use core::ops::Deref;
use core::marker::PhantomData;
use core::mem::transmute;

use stm32f103xx::{AFIO, GPIOB, I2C1, I2C2, i2c1, RCC};

use ::gpio::*;
use ::afio::*;


pub unsafe trait I2C: Deref<Target = i2c1::RegisterBlock> {
}

unsafe impl I2C for I2C1 {

}

unsafe impl I2C for I2C2 {
}


pub enum I2CState {
    Ok,
    /// The I2C module is still busy, so wait for a further response
    Busy,
    /// The module has encountered the error `I2CError`
    Error(I2CError)
}

type I2CWriteState = I2CState;

impl I2CWriteState {
    pub fn cont<F>(&self, mut f : F) -> I2CWriteState 
    where F : FnMut() -> I2CWriteState {
        match *self {
            I2CState::Ok => f(),
            I2CState::Busy => I2CState::Busy,
            I2CState::Error(err) => I2CState::Error(err)
        }
    }
}

type I2CReadState = I2CState;

impl I2CState {
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        match *self {
            I2CState::Ok => true,
            _ => false
        }
    }

    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        match *self {
            I2CState::Busy => true,
            _ => false
        }
    }
}

impl Display for I2CState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            I2CState::Ok => write!(f, "Ok"),
            I2CState::Busy => write!(f, "Busy"),
            I2CState::Error(ref err) => write!(f, "Error<{}>", err)
        }
    }
}


#[derive(Copy, Clone)]
pub enum I2CError {
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


impl I2CError {
    pub fn if_no_err<F>(&self, fun : F) -> I2CState
        where F: Fn() -> I2CState {
        match *self {
            I2CError::None => fun(),
            ref err => I2CState::Error(*err)
        }
    }
}

impl Display for I2CError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            I2CError::None => write!(f, "None"),
            I2CError::Timeout => write!(f,"Timeout"),
            I2CError::AF => write!(f,"AF"),
            I2CError::ARLO => write!(f, "ARLO"),
            I2CError::OVR => write!(f, "OVR"),
            I2CError::BERR => write!(f, "BERR")
        }
    }
}

type_states!(IsConfigured, (NotConfigured, Configured));


pub struct I2cFrequency<'a, S, C>(pub &'a S, PhantomData<C>)
where S: Any + I2C, C: IsConfigured;

impl<'a, S> I2cFrequency<'a, S, NotConfigured> where S: Any + I2C {
    #[inline(always)]
    pub fn set(self, freq : u8) -> I2cFrequency<'a, S, Configured> {
        unsafe {
            self.0.cr2.modify(|_, w| {w.freq().bits(freq)});
            transmute(self) 
        }
    }
}

pub struct I2cTrise<'a, S, C>(pub &'a S, PhantomData<C>)
where S: Any + I2C, C: IsConfigured;

impl<'a, S> I2cTrise<'a, S, NotConfigured> where S: Any + I2C {
    /// set TRISE rise time
    /// for SM mode it is 1000ns for FM mode it is 300ns
    /// assuming T_PLCK1 = 125ns, 300ns / 125 ns ~ 2.4, round up to 3 and then +1
    #[inline(always)]
    pub fn set(self, trise : u8) -> I2cTrise<'a, S, Configured> {
        unsafe {
            self.0.trise.modify(|_, w| {w.trise().bits(trise)});
            transmute(self) 
        }
    }
}


type_states!(BusSpeedMode, (NotSelected, SlowMode, FastMode));
type_group!(BusSpeedModeConfigured, (SlowMode, FastMode));

pub struct I2cBusSpeedMode<'a, S, M>(pub &'a S, PhantomData<M>)
where S: Any + I2C, M : BusSpeedMode;


impl<'a, S> I2cBusSpeedMode<'a, S, NotSelected> where S: Any + I2C {
    #[inline(always)]
    pub fn set_slow_mode(self, ccr: u16) -> I2cBusSpeedMode<'a, S, SlowMode> {
        unsafe {
            self.0.ccr.modify(|_, w| { w.ccr().bits(ccr) });
            transmute(self) 
        }
    }

    #[inline(always)]
    pub fn set_fast_mode(self, ccr : u16) -> I2cBusSpeedMode<'a, S, FastMode> {
        unsafe {
            self.0.ccr.modify(|_, w| { w.f_s().set_bit().ccr().bits(ccr) });
            transmute(self)
        }
    }
}

pub struct I2cBusPorts<'a, S, P>(pub &'a S, PhantomData<P>)
where S: Any + I2C, P: IsConfigured;

impl<'a> I2cBusPorts<'a, I2C1, NotConfigured> {
    #[inline(always)]
    pub fn set_ports_normal<M>(self, 
        _pb6 : GpioPin<'a, GPIOB, Pin6, M, PinCnf3>, 
        _pb7 : GpioPin<'a, GPIOB, Pin7, M, PinCnf3>,
        _afio_i2c : AfioI2C1Peripheral<'a, NotRemapped>) 
        -> I2cBusPorts<'a, I2C1, Configured> where M : PinOutput + PinMode {
            unsafe {
                transmute(self)
            }
        }

    #[inline(always)]
    pub fn set_ports_remapped<M>(self, 
        _pb8 : GpioPin<'a, GPIOB, Pin8, M, PinCnf3>, 
        _pb9 : GpioPin<'a, GPIOB, Pin9, M, PinCnf3>,
        _afio_i2c : AfioI2C1Peripheral<'a, Remapped>) 
        -> I2cBusPorts<'a, I2C1, Configured> where M : PinOutput + PinMode {
            unsafe {
                transmute(self)
            }
        }
}

type_states!(I2cStates, (Start, Read, Write));

pub struct I2cState<'a, S : Any + I2C, ST : I2cStates>(pub &'a S, PhantomData<ST>);

pub enum I2cStateOptions<'a, S: Any + I2C> {
    Started(I2cState<'a, S, Start>),
    CanRead(I2cState<'a, S, Read>),
    CanWrite(I2cState<'a, S, Write>),
    Unknown
}

pub struct I2c<'a, S>(pub &'a S)
where 
    S: Any + I2C; 

impl<'a, S: Any + I2C> I2cState<'a, S, Start> {

    #[inline(always)]
    pub fn write_address(&self, addr : u8, read : bool) {
        let dat = (addr << 1) | (if read {1} else {0});
        self.0.dr.write(|w| unsafe { w.bits(dat as u32) });
    }

    #[inline(always)]
    pub fn stop(&self) {
        self.0.cr1.modify(|_,w| { w.stop().set_bit() });
    }
}

impl<'a, S: Any + I2C> I2cState<'a, S, Write> {

    #[inline(always)]
    pub fn suspend(&self) {
        self.0.cr2.modify(|_, w| {w.itevten().clear_bit()})
    }

    #[inline(always)]
    pub fn write(&self, dat: u8) {
        self.0.dr.write(|w| unsafe { w.bits(dat as u32) });
    }

    #[inline(always)]
    pub fn stop(&self) {
        self.0.cr1.modify(|_,w| { w.stop().set_bit() });
    }
}


impl<'a, S> I2c<'a, S>
where
    S: Any + I2C
{

    pub fn start_init(&self) -> (
        I2cBusSpeedMode<'a, S, NotSelected>,
        I2cFrequency<'a, S, NotConfigured>,
        I2cTrise<'a, S, NotConfigured>,
        I2cBusPorts<'a, S, NotConfigured>,
       ) {
        (
            I2cBusSpeedMode(self.0, PhantomData),
            I2cFrequency(self.0, PhantomData), 
            I2cTrise(self.0, PhantomData),
            I2cBusPorts(self.0, PhantomData)
        )
    }

    pub fn complete_init<M>(&self, 
        _bsm : I2cBusSpeedMode<'a, S, M>, 
        _freq : I2cFrequency<'a, S, Configured>,
        _trise : I2cTrise<'a, S, Configured>,
        _bp : I2cBusPorts<'a, S, Configured>
    ) where M : BusSpeedMode + BusSpeedModeConfigured {
        self.0.cr1.modify(|_, w| {w.pe().set_bit()});
    }

    pub fn listen(&self) {
        self.0.cr2.modify(|_, w| {w.itevten().set_bit().iterren().set_bit()})
    }

    /// Initialize the I2C port.
    /// 
    /// Initializes the GPIO ports of required by the I2C module an then starts it up using Fast Mode (400 kHz).
    /// 
    /// * `remap` - Specifies if the I2C module should use the alternative ports (only available for I2C1)
    pub fn init(&self, remap: bool, afio: &AFIO, gpio: &GPIOB, rcc: &RCC) {
        let i2c = self.0;

        // enable alternate function IO and IO port B
        //rcc.apb2enr.modify(|_, w| {w.afioen().enabled().iopben().enabled()});

        if i2c.get_type_id() == TypeId::of::<I2C1>() {
            rcc.apb1enr.modify(|_, w| {w.i2c1en().enabled()});

            if remap {
                afio.mapr.modify(|_, w| {w.i2c1_remap().set_bit()});
                gpio.crh.modify(|_, w| { w.mode8().output2()});
                gpio.crh.modify(|_, w| { w.cnf8().alt_open()});
                gpio.crh.modify(|_, w| { w.mode9().output2()});
                gpio.crh.modify(|_, w| { w.cnf9().alt_open()});
            } else {
                afio.mapr.modify(|_, w| {w.i2c1_remap().clear_bit()});

                // set RB6 (SCL) and RB7 (SDA) to alternative push pull and 
                // output 2 MHz
                gpio.crl.modify(|_, w| { w.mode6().output2()});
                gpio.crl.modify(|_, w| { w.cnf6().alt_open()});
                gpio.crl.modify(|_, w| { w.mode7().output2()});
                gpio.crl.modify(|_, w| { w.cnf7().alt_open()});
            }
        }


        // set the apb frequency to 8MHz
        i2c.cr2.modify(|_, w| unsafe {w.freq().bits(8)});

        // enable FM mode (400KHz) set duty cycle to 1:1
        // ccr is calculated as T_PLCK1 = 125ns (because 8MHz frequency)
        // so 2500ns / 2 / 125ns = 10
        i2c.ccr.modify(|_, w| unsafe {w.f_s().set_bit().ccr().bits(10)});

        // alternative using 16:9 frequency by setting the duty bit
        //i2c.ccr.modify(|_, w| unsafe {w.f_s().set_bit().duty().set_bit().ccr().bits(1)});

        // set TRISE rise time
        // for SM mode it is 1000ns for FM mode it is 300ns
        // assuming T_PLCK1 = 125ns, 300ns / 125 ns ~ 2.4, round up to 3 and then +1
        i2c.trise.modify(|_, w| unsafe {w.trise().bits(4)});

        // enable the peripheral
        i2c.cr1.modify(|_, w| {w.pe().set_bit()});
    }

    fn get_error(&self, sr1: i2c1::sr1::R) -> I2CError {
        if sr1.timeout().bit_is_set() {
            return I2CError::Timeout
        } else if sr1.af().bit_is_set() {
            return I2CError::AF
        } else if sr1.arlo().bit_is_set() {
            return I2CError::ARLO
        } else if sr1.ovr().bit_is_set() {
            return I2CError::OVR
        } else if sr1.berr().bit_is_set() {
            return I2CError::BERR
        }

        I2CError::None
    }

    // reads the SB (Start Bit) of the Status 1 register
    pub fn is_start_flag_set_async(&self) -> bool {
        let state = self.0.sr1.read();
        let sb = state.sb().bit_is_set();
        sb
    }

    // reads the SB (Start Bit) of the Status 1 register
    pub fn is_start_flag_set(&self) -> I2CState {
        let state = self.0.sr1.read();
        let sb = state.sb().bit_is_set();
        self.get_error(state).if_no_err(|| {
            if sb { I2CState::Ok } else { I2CState::Busy }
        })
    }

    /// read the master mode flag
    pub fn is_msl_flag_set(&self) -> bool {
        self.0.sr2.read().msl().bit_is_set()
    }

    /// Determine if slave address matched
    pub fn is_addr_flag_set(&self) -> I2CState {
        let state = self.0.sr1.read();
        let addr = state.addr().bit_is_set();
        self.get_error(state).if_no_err(|| {
            if addr {
                I2CState::Ok
            } else {
                I2CState::Busy
            }
        }) 
    }

    /// Determine if byte transfer finished (BTF)
    pub fn is_byte_transfer_finished(&self) -> I2CState {
        let state = self.0.sr1.read();
        let btf = state.btf().bit_is_set();
        self.get_error(state).if_no_err(|| {
            if btf {I2CState::Ok} else {I2CState::Busy}
        })
    }

    /// Determine if data register is empty (TxE)
    pub fn is_data_register_empty(&self) -> I2CState {
        let state = self.0.sr1.read();
        let txe = state.tx_e().bit_is_set();
        self.get_error(state).if_no_err(|| {
            if txe {I2CState::Ok} else { I2CState::Busy }
        })
    }

    /// Determine if a byte has been received which can be read from the data register (RxNE)
    pub fn is_data_register_not_empty(&self) -> I2CState {
        let state = self.0.sr1.read();
        let rxne = state.rx_ne().bit_is_set();
        self.get_error(state).if_no_err(|| {
            if rxne { I2CState::Ok } else { I2CState::Busy }
        })
    }
    #[inline(always)]
    pub fn write_data(&self, dat : u8) -> I2CWriteState {
        self.0.dr.write(|w| unsafe { w.bits(dat as u32) });
        self.poll_loop(|| {self.is_data_register_empty()})
    }

    #[inline(always)]
    pub fn start_write_async(&self, addr : u8) {
        let dat = (addr << 1) | 1;
        self.0.dr.write(|w| unsafe { w.bits(dat as u32) });
    }

    #[inline(always)]
    pub fn write_async(&self, dat: u8) {
        self.0.dr.write(|w| unsafe { w.bits(dat as u32) });
    }

    #[inline(always)]
    pub fn start_read(&self, addr : u8) {
        let dat = (addr << 1) | 0;
        self.0.dr.write(|w| unsafe { w.bits(dat as u32) });
    }

    #[inline(always)]
    pub fn read_data(&self, out: &mut u8) -> I2CReadState {
        let state = self.poll_loop(|| { self.is_data_register_not_empty() });
        if state.is_ok() {
            *out = self.0.dr.read().bits() as u8;
        }
        state
    }

    #[inline(always)]
    pub fn read_last_data(&self, out: &mut u8) -> I2CReadState {
        self.0.cr1.modify(|_,w| {w.ack().clear_bit()});
        self.read_data(out)
    }

    pub fn poll_loop<T>(&self, fun: T) -> I2CState 
        where T : Fn() -> I2CState {
        loop {
            let state = fun();
            if !state.is_busy() { return state }
        }
    }

    /// Send the start signal and write the `addr` to the bus.
    /// 
    /// `read` specifies if it is a read request (`true`) or a write request (`false`)
    pub fn start_polling(&self, addr : u8, read : bool) -> I2CState {
        self.enable_start();

        self.poll_loop(|| { self.is_start_flag_set() }).cont(|| {
            self.write_data((addr << 1) + (if read { 1 } else { 0 }));
            if read {
                self.0.cr1.modify(|_,w| {w.pos().set_bit().ack().set_bit()});
            }
            self.poll_loop(|| {self.is_addr_flag_set()})
        }).cont(|| {
            self.0.sr2.read();
            I2CState::Ok
        })
    } 

    #[inline(always)]
    pub fn start_write_polling(&self, addr : u8) -> I2CWriteState {
        self.start_polling(addr, false)
    }

    #[inline(always)]
    pub fn start_read_polling(&self, addr: u8) -> I2CReadState {
        let state = self.start_polling(addr, true);
        state
    }

    pub fn stop(&self) -> I2CState {
        self.enable_stop();
        I2CState::Ok
    }

    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        let i2c = self.0;
        i2c.sr2.read().busy().bit_is_set()
    }

    #[inline(always)]
    pub fn enable_start(&self) {
        let i2c = self. 0;
        i2c.cr1.modify(|_, w| {w.start().set_bit()});
    }

    #[inline(always)]
    fn enable_stop(&self) {
        let i2c = self.0;
        i2c.cr1.modify(|_, w| {w.stop().set_bit()});
    }


    pub fn get_state(&self) -> I2cStateOptions<'a, S> {
        let sr1 = self.0.sr1.read();

        if sr1.sb().bit_is_set() {
            I2cStateOptions::Started(I2cState(&self.0, PhantomData))
        } else if sr1.addr().bit_is_set() {
            let _b = self.0.sr2.read();
            I2cStateOptions::CanWrite(I2cState(&self.0, PhantomData))
        } else if sr1.tx_e().bit_is_set() {
            I2cStateOptions::CanWrite(I2cState(&self.0, PhantomData))
        } else if sr1.rx_ne().bit_is_set() {
            I2cStateOptions::CanRead(I2cState(&self.0, PhantomData))
        } else {
            I2cStateOptions::Unknown
        }
    }
}