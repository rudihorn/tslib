
#[allow(unused_imports)]
use common;

use core::mem::transmute;
use core::marker::PhantomData;
use core::ops::Deref;

use rcc;
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

pub struct GpioPin<'a, G:'a, P, M, C>(pub &'a G, PhantomData<(P, M, C)>)
where G: GPIO, P: Pins, M: PinMode, C: PinCnf;
pub type GpioPinDefault<'a, Bank, Pin> = GpioPin<'a, Bank, Pin, Input, PinCnf1>;

pub trait GpioPinRaw {
    fn set_mode_val_new(&self, value : u8);
}

macro_rules! per_pin {
    ($pin : ident, $num : expr, $reg : ident, $modeop : ident) => {
        impl<'a, G, M, C> GpioPinRaw for GpioPin<'a, G, $pin, M, C,>
        where G: GPIO, M: PinMode, C: PinCnf {
            fn set_mode_val_new(&self, value : u8) {
                (self.0).$reg.modify(|_,w| w.$modeop().bits(value));
            }
        }
    };
}

per_pin!(Pin0, 0, crl, mode0);
per_pin!(Pin1, 1, crl, mode1);
per_pin!(Pin2, 2, crl, mode2);
per_pin!(Pin3, 3, crl, mode3);
per_pin!(Pin4, 4, crl, mode4);
per_pin!(Pin5, 5, crl, mode5);
per_pin!(Pin6, 6, crl, mode6);
per_pin!(Pin7, 7, crl, mode7);
per_pin!(Pin8, 8, crh, mode8);
per_pin!(Pin9, 9, crh, mode9);
per_pin!(Pin10, 10, crh, mode10);
per_pin!(Pin11, 11, crh, mode11);
per_pin!(Pin12, 12, crh, mode12);
per_pin!(Pin13, 13, crh, mode13);
per_pin!(Pin14, 14, crh, mode14);
per_pin!(Pin15, 15, crh, mode15);

macro_rules! common_funs {
    ($(($num : expr, $reg : ident, $modeop : ident, $cnf:ident)), *) => {

        impl<'a, G, P, M, C> GpioPin<'a, G, P, M, C>
        where G: GPIO, M: PinMode, C: PinCnf, P: Pins + PinNr {
            #[inline(always)]
            fn set_mode_val(&self, value : u8) {
                match P::nr() {
                    $(
                        $num => (self.0).$reg.modify(|_, w| w.$modeop().bits(value)),
                    )*
                    _ => ()
                }
            }

            #[inline(always)]
            fn set_cnf_val(&self, value : u8) {
                match P::nr() {
                    $(
                        $num => (self.0).$reg.modify(|_, w| w.$cnf().bits(value)),
                    )*
                    _ => ()
                }
            }
        }
    }
}

common_funs!(
    (0, crl, mode0, cnf0),
    (1, crl, mode1, cnf1),
    (2, crl, mode2, cnf2),
    (3, crl, mode3, cnf3),
    (4, crl, mode4, cnf4),
    (5, crl, mode5, cnf5),
    (6, crl, mode6, cnf6),
    (7, crl, mode7, cnf7),
    (8, crh, mode8, cnf8),
    (9, crh, mode9, cnf9),
    (10, crh, mode10, cnf10),
    (11, crh, mode11, cnf11),
    (12, crh, mode12, cnf12),
    (13, crh, mode13, cnf13),
    (14, crh, mode14, cnf14),
    (15, crh, mode15, cnf15)
);

impl<'a, G, P, M, C> GpioPin<'a, G, P, M, C>
where G: GPIO, M: PinMode, C: PinCnf, P: Pins + PinNr {
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_10MHz(self) -> GpioPin<'a, G, P, Output10, C> {
        self.set_mode_val(0b01);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_2MHz(self) -> GpioPin<'a, G, P, Output2, C> {
        self.set_mode_val(0b10);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_50MHz(self) -> GpioPin<'a, G, P, Output50, C> {
        self.set_mode_val(0b11);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_input(self) -> GpioPin<'a, G, P, Input, C> {
        self.set_mode_val(0b00);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_0(self) -> GpioPin<'a, G, P, M, PinCnf0> {
        self.set_cnf_val(0b00);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_1(self) -> GpioPin<'a, G, P, M, PinCnf1> {
        self.set_cnf_val(0b01);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_2(self) -> GpioPin<'a, G, P, M, PinCnf2> {
        self.set_cnf_val(0b10);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_3(self) -> GpioPin<'a, G, P, M, PinCnf3> {
        self.set_cnf_val(0b11);
        GpioPin(self.0, PhantomData)
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
    pub fn get_pins(self, _rcc: rcc::RccPeripheral<G, rcc::Enabled>) -> (
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
