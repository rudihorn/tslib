
#[allow(unused_imports)]
use common;

use core::marker::PhantomData;
use core::ops::Deref;

use rcc;
use stm32f103xx::{gpioa, GPIOA, GPIOB, GPIOC, GPIOD};

pub unsafe trait GPIO : Deref<Target = gpioa::RegisterBlock> {
    fn ptr() -> *const gpioa::RegisterBlock;
}

macro_rules! gpio_block {
    ($block:ident) => {
        unsafe impl GPIO for $block {
            fn ptr() -> *const gpioa::RegisterBlock {
                $block::ptr()
            }
        }
    }
}

gpio_block!(GPIOA);
gpio_block!(GPIOB);
gpio_block!(GPIOC);
gpio_block!(GPIOD);

type_states!(IsEnabled, (NotEnabled, Enabled));

type_states!(Pins, (Pin0, Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7, Pin8, Pin9, Pin10, Pin11, Pin12, Pin13, Pin14, Pin15));
type_group!(PinsLow, (Pin0, Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7));
pub trait PinNr { fn nr() -> u8; }

macro_rules! pin_nr{
    ($pin:ident, $nr:expr) => {
        impl PinNr for $pin { #[inline(always)] fn nr() -> u8 { $nr } }
    }
}

pin_nr!(Pin0, 0);
pin_nr!(Pin1, 1);
pin_nr!(Pin2, 2);
pin_nr!(Pin3, 3);
pin_nr!(Pin4, 4);
pin_nr!(Pin5, 5);
pin_nr!(Pin6, 6);
pin_nr!(Pin7, 7);
pin_nr!(Pin8, 8);
pin_nr!(Pin9, 9);
pin_nr!(Pin10, 10);
pin_nr!(Pin11, 11);
pin_nr!(Pin12, 12);
pin_nr!(Pin13, 13);
pin_nr!(Pin14, 14);
pin_nr!(Pin15, 15);

type_group!(PinsHigh, (Pin8, Pin9, Pin10, Pin11, Pin12, Pin13, Pin14, Pin15));

type_states!(PinMode, (Input, Output10, Output2, Output50));
type_group!(PinOutput, (Output10, Output2, Output50));
type_states!(PinCnf, (PinCnf0, PinCnf1, PinCnf2, PinCnf3));

pub struct GpioPin<G, P, M, C>(PhantomData<G>, PhantomData<(P, M, C)>)
where G: GPIO, P: Pins, M: PinMode, C: PinCnf;
pub type GpioPinDefault<Bank, Pin> = GpioPin<Bank, Pin, Input, PinCnf1>;

impl<G, P, M, C> GpioPin<G, P, M, C>
where G: GPIO, M: PinMode, C: PinCnf, P: Pins + PinNr {
    #[inline(always)]
    fn gpio<'a>(&self) -> &'a gpioa::RegisterBlock {
        unsafe { &(*G::ptr()) }
    }
}

macro_rules! common_funs {
    ($(($num : expr, $reg : ident, $modeop : ident, $cnf:ident)), *) => {

        impl<G, P, M, C> GpioPin<G, P, M, C>
        where G: GPIO, M: PinMode, C: PinCnf, P: Pins + PinNr {
            #[inline(always)]
            fn set_mode_val(&mut self, value : u8) {
                match P::nr() {
                    $(
                        
                        $num => self.gpio().$reg.modify(|_, w| w.$modeop().bits(value)),
                    )*
                    _ => ()
                }
            }

            #[inline(always)]
            fn set_cnf_val(&mut self, value : u8) {
                match P::nr() {
                    $(
                        $num => self.gpio().$reg.modify(|_, w| w.$cnf().bits(value)),
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

impl<G, P, M, C> GpioPin<G, P, M, C>
where G: GPIO, M: PinMode, C: PinCnf, P: Pins + PinNr {
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_10MHz(mut self) -> GpioPin<G, P, Output10, C> {
        self.set_mode_val(0b01);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_2MHz(mut self) -> GpioPin<G, P, Output2, C> {
        self.set_mode_val(0b10);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_50MHz(mut self) -> GpioPin<G, P, Output50, C> {
        self.set_mode_val(0b11);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    pub fn set_input(mut self) -> GpioPin<G, P, Input, C> {
        self.set_mode_val(0b00);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    pub fn set_cnf_0(mut self) -> GpioPin<G, P, M, PinCnf0> {
        self.set_cnf_val(0b00);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    pub fn set_cnf_1(mut self) -> GpioPin<G, P, M, PinCnf1> {
        self.set_cnf_val(0b01);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    pub fn set_cnf_2(mut self) -> GpioPin<G, P, M, PinCnf2> {
        self.set_cnf_val(0b10);
        GpioPin(self.0, PhantomData)
    }

    #[inline(always)]
    pub fn set_cnf_3(mut self) -> GpioPin<G, P, M, PinCnf3> {
        self.set_cnf_val(0b11);
        GpioPin(self.0, PhantomData)
    }
}

impl<G, P, C> GpioPin<G, P, Input, C>
where G: GPIO, C: PinCnf, P: Pins + PinNr {
    #[inline(always)]
    pub fn set_analog(self) -> GpioPin<G, P, Input, PinCnf0>{
        self.set_cnf_0()
    }

    #[inline(always)]
    pub fn set_floating_input(self) -> GpioPin<G, P, Input, PinCnf1>{
        self.set_cnf_1()
    }

    #[inline(always)]
    pub fn set_pull_up_down(self) -> GpioPin<G, P, Input, PinCnf2> {
        self.set_cnf_2()
    }

    #[inline(always)]
    pub fn set_pull_down(self) -> GpioPin<G, P, Input, PinCnf2> {
        let p = self.set_pull_up_down();
        unsafe {
            p.gpio().odr.write(|w| w.bits(1 << P::nr()));
        }
        p
    }

    #[inline(always)]
    pub fn read(&self) -> bool {
        (self.gpio().idr.read().bits() & (1 << P::nr())) != 0
    }
}

impl<G, P, M, C> GpioPin<G, P, M, C>
where G: GPIO, M: PinOutput + PinMode, C: PinCnf, P: Pins + PinNr {
    #[inline(always)]
    pub fn set_alt_output_open_drain(self) -> GpioPin<G, P, M, PinCnf3>{
        self.set_cnf_3()
    }

    #[inline(always)]
    pub fn set_alt_output_push_pull(self) -> GpioPin<G, P, M, PinCnf2>{
        self.set_cnf_2()
    }

    #[inline(always)]
    pub fn set_output_open_drain(self) -> GpioPin<G, P, M, PinCnf1>{
        self.set_cnf_1()
    }

    #[inline(always)]
    pub fn set_output_push_pull(self) -> GpioPin<G, P, M, PinCnf0>{
        self.set_cnf_0()
    }
}

impl<G, P, M, C> GpioPin<G, P, M, C> 
where G:GPIO, M:PinOutput + PinMode, P:Pins + PinNr, C:PinCnf {
    pub fn set(&mut self, high:bool) {
        unsafe {
            self.gpio().bsrr.write(|w| w.bits(1 << P::nr() + (if high { 0 } else { 16 })));
        }
    }
}

macro_rules! get_pins_macro{
    ($($pin:ident),*) => {
        #[inline(always)]
        pub fn get_pins(self, _rcc: rcc::RccPeripheral<G, rcc::Enabled>) -> (
            $(GpioPinDefault<G, $pin>,)*
        ) {
            (
                $(GpioPin(PhantomData, PhantomData) as GpioPinDefault<G, $pin>,)*
            )
        }
    }
}

pub struct Gpio<G>(PhantomData<G>) 
where G: GPIO;

impl<G> Gpio<G> where G: GPIO {
    pub fn new(_gpio: G) -> Self {
        Gpio(PhantomData)
    }

    get_pins_macro!(
        Pin0, Pin1, Pin2, Pin3, 
        Pin4, Pin5, Pin6, Pin7, 
        Pin8, Pin9, Pin10, Pin11, 
        Pin12, Pin13, Pin14, Pin15
        );
}

