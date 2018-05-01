
#[allow(unused_imports)]
use common;

use core::mem::transmute;
use core::marker::PhantomData;
use core::ops::Deref;

use rcc::{RccIOPeripheral, PeripheralEnabled};
use stm32f103xx::{gpioa, GPIOA, GPIOB, GPIOC, GPIOD};



pub unsafe trait GPIO : Deref<Target = gpioa::RegisterBlock> {
}

unsafe impl GPIO for GPIOA {}
unsafe impl GPIO for GPIOB {}
unsafe impl GPIO for GPIOC {}
unsafe impl GPIO for GPIOD {}

type_states!(IsEnabled, (NotEnabled, Enabled));

type_states!(Pins, (Pin0, Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7, Pin8, Pin9, Pin10, Pin11, Pin12, Pin13, Pin14, Pin15));
type_group!(PinsLow, (Pin0, Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7));
pub trait PinNr { fn nr() -> u8; }
impl PinNr for Pin0 { #[inline(always)] fn nr() -> u8 { 0 } }
impl PinNr for Pin1 { #[inline(always)] fn nr() -> u8 { 1 } }
impl PinNr for Pin2 { #[inline(always)] fn nr() -> u8 { 2 } }
impl PinNr for Pin3 { #[inline(always)] fn nr() -> u8 { 3 } }
impl PinNr for Pin4 { #[inline(always)] fn nr() -> u8 { 4 } }
impl PinNr for Pin5 { #[inline(always)] fn nr() -> u8 { 5 } }
impl PinNr for Pin6 { #[inline(always)] fn nr() -> u8 { 6 } }
impl PinNr for Pin7 { #[inline(always)] fn nr() -> u8 { 7 } }
impl PinNr for Pin8 { #[inline(always)] fn nr() -> u8 { 8 } }
impl PinNr for Pin9 { #[inline(always)] fn nr() -> u8 { 9 } }
impl PinNr for Pin10 { #[inline(always)] fn nr() -> u8 { 10 } }
impl PinNr for Pin11 { #[inline(always)] fn nr() -> u8 { 11 } }
impl PinNr for Pin12 { #[inline(always)] fn nr() -> u8 { 12 } }
impl PinNr for Pin13 { #[inline(always)] fn nr() -> u8 { 13 } }
impl PinNr for Pin14 { #[inline(always)] fn nr() -> u8 { 14 } }
impl PinNr for Pin15 { #[inline(always)] fn nr() -> u8 { 15 } }

type_group!(PinsHigh, (Pin8, Pin9, Pin10, Pin11, Pin12, Pin13, Pin14, Pin15));

type_states!(PinMode, (Input, Output10, Output2, Output50));
type_group!(PinOutput, (Output10, Output2, Output50));
type_states!(PinCnf, (PinCnf0, PinCnf1, PinCnf2, PinCnf3));

pub type GpioPinDefault<'a, Bank : GPIO, Pin: Pins> = GpioPin<'a, Bank, Pin, Input, PinCnf1>;

pub struct GpioPin<'a, G:'a, P, M, C>(pub &'a G, PhantomData<(P, M, C)>)
where G: GPIO, P: Pins, M: PinMode, C: PinCnf;

impl<'a, G, P, M, C> GpioPin<'a, G, P, M, C>
where G: GPIO, M: PinMode, C: PinCnf, P: Pins + PinNr {
    #[inline(always)]
    fn set_mode_val(&self, value : u8) {
        const MASK: u8 = 0b11;
        let higher = P::nr() >= 8;
        let offset = if higher { P::nr() - 8 } else { P::nr() } * 4;
        unsafe {
            let reg = if higher {
                self.0.crh.modify(|r,w| { w.bits((r.bits() & !((MASK as u32) << offset)) | (((value & MASK) as u32) << offset)) })
            } else {
                self.0.crl.modify(|r,w| { w.bits((r.bits() & !((MASK as u32) << offset)) | (((value & MASK) as u32) << offset)) })
            };
        }
    }

    #[inline(always)]
    fn set_cnf_val(&self, value : u8) {
        const MASK: u8 = 0b11;
        let higher = P::nr() >= 8;
        let offset = 2 + if higher { P::nr() - 8 } else { P::nr() } * 4;
        unsafe {
            let reg = if higher {
                self.0.crh.modify(|r,w| { w.bits((r.bits() & !((MASK as u32) << offset)) | (((value & MASK) as u32) << offset)) })
            } else {
                self.0.crl.modify(|r,w| { w.bits((r.bits() & !((MASK as u32) << offset)) | (((value & MASK) as u32) << offset)) })
            };
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_10MHz(self) -> GpioPin<'a, G, P, Output10, C> {
        self.set_mode_val(0b01);
        unsafe { transmute(self) }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_2MHz(self) -> GpioPin<'a, G, P, Output2, C> {
        self.set_mode_val(0b10);
        unsafe { transmute(self) }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_50MHz(self) -> GpioPin<'a, G, P, Output50, C> {
        self.set_mode_val(0b11);
        unsafe { transmute(self) }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_input(self) -> GpioPin<'a, G, P, Input, C> {
        self.set_mode_val(0b00);
        unsafe { transmute(self) }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_0(self) -> GpioPin<'a, G, P, M, PinCnf0> {
        self.set_cnf_val(0b00);
        unsafe { transmute(self) }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_1(self) -> GpioPin<'a, G, P, M, PinCnf1> {
        self.set_cnf_val(0b01);
        unsafe { transmute(self) }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_2(self) -> GpioPin<'a, G, P, M, PinCnf2> {
        self.set_cnf_val(0b10);
        unsafe { transmute(self) }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_3(self) -> GpioPin<'a, G, P, M, PinCnf3> {
        self.set_cnf_val(0b11);
        unsafe { transmute(self) }
    }
}

impl<'a, G, P, C> GpioPin<'a, G, P, Input, C>
where G: GPIO, C: PinCnf, P: Pins + PinNr {
    #[inline(always)]
    pub fn set_analog(self) -> GpioPin<'a, G, P, Input, PinCnf0>{
        self.set_cnf_0()
    }

    #[inline(always)]
    pub fn set_floating_input(self) -> GpioPin<'a, G, P, Input, PinCnf1>{
        self.set_cnf_1()
    }

    #[inline(always)]
    pub fn set_pull_up_down(self) -> GpioPin<'a, G, P, Input, PinCnf2> {
        self.set_cnf_2()
    }

    #[inline(always)]
    pub fn set_pull_down(self) -> GpioPin<'a, G, P, Input, PinCnf2> {
        unsafe {
            self.0.odr.write(|w| w.bits(1 << P::nr()));
            transmute(self)
        }
    }

    #[inline(always)]
    pub fn read(&self) -> bool {
        (self.0.idr.read().bits() & (1 << P::nr())) != 0
    }
}

impl<'a, G, P, M, C> GpioPin<'a, G, P, M, C>
where G: GPIO, M: PinOutput + PinMode, C: PinCnf, P: Pins + PinNr {
    #[inline(always)]
    pub fn set_alt_output_open_drain(self) -> GpioPin<'a, G, P, M, PinCnf3>{
        self.set_cnf_3()
    }

    #[inline(always)]
    pub fn set_alt_output_push_pull(self) -> GpioPin<'a, G, P, M, PinCnf2>{
        self.set_cnf_2()
    }

    #[inline(always)]
    pub fn set_output_open_drain(self) -> GpioPin<'a, G, P, M, PinCnf1>{
        self.set_cnf_1()
    }

    #[inline(always)]
    pub fn set_output_push_pull(self) -> GpioPin<'a, G, P, M, PinCnf0>{
        self.set_cnf_0()
    }
}

impl<'a, G, P, M, C> GpioPin<'a, G, P, M, C> 
where G:GPIO, M:PinOutput + PinMode, P:Pins + PinNr, C:PinCnf {
    pub fn set(&mut self, high:bool) {
        unsafe {
            self.0.bsrr.write(|w| w.bits(1 << P::nr() + (if high { 0 } else { 16 })));
        }
    }
}

pub struct Gpio<'a, G:'a>(pub &'a G) 
where G: GPIO;

impl<'a, G> Gpio<'a, G> where G: GPIO {
    #[inline(always)]
    pub fn get_pins(self, _rcc: RccIOPeripheral<'a, G, PeripheralEnabled>) -> (
        GpioPinDefault<'a, G, Pin0>, 
        GpioPinDefault<'a, G, Pin1>, 
        GpioPinDefault<'a, G, Pin2>, 
        GpioPinDefault<'a, G, Pin3>, 
        GpioPinDefault<'a, G, Pin4>, 
        GpioPinDefault<'a, G, Pin5>, 
        GpioPinDefault<'a, G, Pin6>, 
        GpioPinDefault<'a, G, Pin7>, 
        GpioPinDefault<'a, G, Pin8>, 
        GpioPinDefault<'a, G, Pin9>, 
        GpioPinDefault<'a, G, Pin10>, 
        GpioPinDefault<'a, G, Pin11>, 
        GpioPinDefault<'a, G, Pin12>, 
        GpioPinDefault<'a, G, Pin13>, 
        GpioPinDefault<'a, G, Pin14>, 
        GpioPinDefault<'a, G, Pin15> 
    ) {
        (
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
            GpioPin(self.0, PhantomData),
        )
    }
}
