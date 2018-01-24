
#[allow(unused_imports)]
#[macro_use]
use common;

use core::mem::transmute;
use core::marker::PhantomData;
use core::ops::Deref;

use blue_pill::stm32f103xx::{gpioa, GPIOA, GPIOB, GPIOC, GPIOD};





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

impl<'a, G, P, M, C> GpioPin<'a, G, P, M, C>
where G: GPIO, M: PinMode, C: PinCnf, P: Pins + PinsHigh + PinNr {
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_10MHz_h(self) -> GpioPin<'a, G, P, Output10, C> {
        const VALUE: u8 = 0b01;
        const MASK: u8 = 0b11;
        let offset = (P::nr() - 8) * 4;
        unsafe {
            self.0.crh.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            });
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_2MHz_h(self) -> GpioPin<'a, G, P, Output2, C> {
        const VALUE: u8 = 0b10;
        const MASK: u8 = 0b11;
        let offset = (P::nr() - 8) * 4;
        unsafe {
            self.0.crh.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_50MHz_h(self) -> GpioPin<'a, G, P, Output50, C> {
        const VALUE: u8 = 0b11;
        const MASK: u8 = 0b11;
        let offset = (P::nr() - 8) * 4;
        unsafe {
            self.0.crh.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_input_h(self) -> GpioPin<'a, G, P, Input, C> {
        const VALUE: u8 = 0b00;
        const MASK: u8 = 0b11;
        let offset = (P::nr() - 8) * 4;
        unsafe {
            self.0.crh.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_0_h(self) -> GpioPin<'a, G, P, M, PinCnf0> {
        const VALUE: u8 = 0b00;
        const MASK: u8 = 0b11;
        let offset = 2 + (P::nr() - 8) * 4;
        unsafe {
            self.0.crh.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_1_h(self) -> GpioPin<'a, G, P, M, PinCnf1> {
        const VALUE: u8 = 0b01;
        const MASK: u8 = 0b11;
        let offset = 2 + (P::nr() - 8) * 4;
        unsafe {
            self.0.crh.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_2_h(self) -> GpioPin<'a, G, P, M, PinCnf2> {
        const VALUE: u8 = 0b10;
        const MASK: u8 = 0b11;
        let offset = 2 + (P::nr() - 8) * 4;
        unsafe {
            self.0.crh.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_3_h(self) -> GpioPin<'a, G, P, M, PinCnf3> {
        const VALUE: u8 = 0b11;
        const MASK: u8 = 0b11;
        let offset = 2 + (P::nr() - 8) * 4;
        unsafe {
            self.0.crh.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }
}

impl<'a, G, P, M, C> GpioPin<'a, G, P, M, C>
where G: GPIO, M: PinMode, C: PinCnf, P: Pins + PinsLow + PinNr {
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_10MHz(self) -> GpioPin<'a, G, P, Output10, C> {
        const VALUE: u8 = 0b01;
        const MASK: u8 = 0b11;
        let offset = P::nr() * 4;
        unsafe {
            self.0.crl.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            });
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_2MHz(self) -> GpioPin<'a, G, P, Output2, C> {
        const VALUE: u8 = 0b10;
        const MASK: u8 = 0b11;
        let offset = P::nr() * 4;
        unsafe {
            self.0.crl.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_output_50MHz(self) -> GpioPin<'a, G, P, Output50, C> {
        const VALUE: u8 = 0b11;
        const MASK: u8 = 0b11;
        let offset = P::nr() * 4;
        unsafe {
            self.0.crl.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_input(self) -> GpioPin<'a, G, P, Input, C> {
        const VALUE: u8 = 0b00;
        const MASK: u8 = 0b11;
        let offset = P::nr() * 4;
        unsafe {
            self.0.crl.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_0(self) -> GpioPin<'a, G, P, M, PinCnf0> {
        const VALUE: u8 = 0b00;
        const MASK: u8 = 0b11;
        let offset = 2 + P::nr() * 4;
        unsafe {
            self.0.crl.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_1(self) -> GpioPin<'a, G, P, M, PinCnf1> {
        const VALUE: u8 = 0b01;
        const MASK: u8 = 0b11;
        let offset = 2 + P::nr() * 4;
        unsafe {
            self.0.crl.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_2(self) -> GpioPin<'a, G, P, M, PinCnf2> {
        const VALUE: u8 = 0b10;
        const MASK: u8 = 0b11;
        let offset = 2 + P::nr() * 4;
        unsafe {
            self.0.crl.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }

    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn set_cnf_3(self) -> GpioPin<'a, G, P, M, PinCnf3> {
        const VALUE: u8 = 0b11;
        const MASK: u8 = 0b11;
        let offset = 2 + P::nr() * 4;
        unsafe {
            self.0.crl.modify(|r,w| { 
                w.bits((r.bits() & !((MASK as u32) << offset)) | (((VALUE & MASK) as u32) << offset) )
            }); 
            transmute(self)
        }
    }
}

impl<'a, G, P, C> GpioPin<'a, G, P, Input, C>
where G: GPIO, C: PinCnf, P: Pins + PinsLow + PinNr {
    #[inline(always)]
    pub fn set_floating_input(self) -> GpioPin<'a, G, P, Input, PinCnf1>{
        self.set_cnf_1()
    }
}

impl<'a, G, P, C> GpioPin<'a, G, P, Input, C>
where G: GPIO, C: PinCnf, P: Pins + PinsHigh + PinNr {
    #[inline(always)]
    pub fn set_floating_input_h(self) -> GpioPin<'a, G, P, Input, PinCnf1>{
        self.set_cnf_1_h()
    }
}

impl<'a, G, P, M, C> GpioPin<'a, G, P, M, C>
where G: GPIO, M: PinOutput + PinMode, C: PinCnf, P: Pins + PinsLow + PinNr {
    #[inline(always)]
    pub fn set_alt_output_open_drain(self) -> GpioPin<'a, G, P, M, PinCnf3>{
        self.set_cnf_3()
    }

    #[inline(always)]
    pub fn set_alt_output_push_pull(self) -> GpioPin<'a, G, P, M, PinCnf2>{
        self.set_cnf_2()
    }
}

impl<'a, G, P, M, C> GpioPin<'a, G, P, M, C>
where G: GPIO, M: PinOutput + PinMode, C: PinCnf, P: Pins + PinsHigh + PinNr {
    #[inline(always)]
    pub fn set_alt_output_open_drain_h(self) -> GpioPin<'a, G, P, M, PinCnf3>{
        self.set_cnf_3_h()
    }

    #[inline(always)]
    pub fn set_alt_output_push_pull_h(self) -> GpioPin<'a, G, P, M, PinCnf2>{
        self.set_cnf_2_h()
    }
}

pub struct Gpio<'a, G:'a>(pub &'a G) 
where G: GPIO;

impl<'a, G> Gpio<'a, G> where G: GPIO {
    #[inline(always)]
    pub fn get_pins(self) -> (
        GpioPin<'a, G, Pin0, Input, PinCnf1>, 
        GpioPin<'a, G, Pin1, Input, PinCnf1>, 
        GpioPin<'a, G, Pin2, Input, PinCnf1>, 
        GpioPin<'a, G, Pin3, Input, PinCnf1>, 
        GpioPin<'a, G, Pin4, Input, PinCnf1>, 
        GpioPin<'a, G, Pin5, Input, PinCnf1>, /* ... */
        GpioPin<'a, G, Pin6, Input, PinCnf1>, 
        GpioPin<'a, G, Pin7, Input, PinCnf1>, 
        GpioPin<'a, G, Pin8, Input, PinCnf1>, 
        GpioPin<'a, G, Pin9, Input, PinCnf1>, 
        GpioPin<'a, G, Pin10, Input, PinCnf1>, 
        GpioPin<'a, G, Pin11, Input, PinCnf1>, 
        GpioPin<'a, G, Pin12, Input, PinCnf1>, 
        GpioPin<'a, G, Pin13, Input, PinCnf1>, 
        GpioPin<'a, G, Pin14, Input, PinCnf1>, 
        GpioPin<'a, G, Pin15, Input, PinCnf1> 
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