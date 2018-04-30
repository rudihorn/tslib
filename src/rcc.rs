

#[allow(unused_imports)]
use common;

use ::core::marker::PhantomData;
use ::core::mem::transmute;

use stm32f103xx::{RCC, GPIOA, GPIOB, GPIOC, I2C1, I2C2, TIM2, SPI1, SPI2, USART1, USART2};
use gpio::GPIO;
use spi::SPI;

type_states!(PeripheralState, (PeripheralDisabled, PeripheralEnabled));

pub struct RccUSARTPeripheral<'a, U, S: PeripheralState>(pub &'a RCC, PhantomData<(U, S)>);

impl <'a> RccUSARTPeripheral<'a, USART1, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable_usart1(self) -> RccUSARTPeripheral<'a, USART1, PeripheralEnabled> {
        unsafe {
            self.0.apb2enr.modify(|_, w| w.usart1en().enabled());
            transmute(self)
        }
    }
}

impl <'a> RccUSARTPeripheral<'a, USART2, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable_usart2(self) -> RccUSARTPeripheral<'a, USART2, PeripheralEnabled> {
        unsafe {
            self.0.apb1enr.modify(|_, w| w.usart2en().enabled());
            transmute(self)
        }
    }
}

pub struct RccTIMPeripheral<'a, T, S: PeripheralState>(pub &'a RCC, PhantomData<(T, S)>);

impl<'a> RccTIMPeripheral<'a, TIM2, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable_tim2(self) -> RccTIMPeripheral<'a, TIM2, PeripheralEnabled> {
        unsafe {
            self.0.apb1enr.modify(|_, w| w.tim2en().enabled());
            transmute(self)
        }
    }
}

impl<'a, P> RccTIMPeripheral<'a, TIM2, P> where P : PeripheralState {
    #[inline(always)]
    pub fn reset(self) -> RccTIMPeripheral<'a, TIM2, P> {
        unsafe {
            self.0.apb1rstr.modify(|_, w| w.tim2rst().set_bit());
            self.0.apb1rstr.modify(|_, w| w.tim2rst().clear_bit());
            transmute(self)
        }
    }
}

pub struct RccI2CPeripheral<'a, I2C, S : PeripheralState>(pub &'a RCC, PhantomData<(I2C, S)>);

impl<'a> RccI2CPeripheral<'a, I2C1, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable_i2c1(self) -> RccI2CPeripheral<'a, I2C1, PeripheralEnabled> {
        unsafe {
            self.0.apb1enr.modify(|_, w| {w.i2c1en().enabled()});
            transmute(self)
        }
    }
}

impl<'a> RccI2CPeripheral<'a, I2C2, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable_i2c2(self) -> RccI2CPeripheral<'a, I2C2, PeripheralEnabled> {
        unsafe {
            self.0.apb1enr.modify(|_, w| {w.i2c2en().enabled()});
            transmute(self)
        }
    }
}

pub struct RccAFIOPeripheral<'a, S : PeripheralState>(pub &'a RCC, PhantomData<S>);

impl<'a> RccAFIOPeripheral<'a, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable(self) -> RccAFIOPeripheral<'a, PeripheralEnabled> {
        unsafe {
            self.0.apb2enr.modify(|_, w| {w.afioen().enabled()});
            transmute(self)
        }
    }
}

pub struct RccIOPeripheral<'a, G : GPIO, S : PeripheralState>(pub &'a RCC, PhantomData<(S, G)>);

impl<'a> RccIOPeripheral<'a, GPIOA, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable_gpioa(self) -> RccIOPeripheral<'a, GPIOA, PeripheralEnabled> {
        unsafe {
            self.0.apb2enr.modify(|_, w| {w.iopaen().enabled()});
            transmute(self)
        }
    }
}
impl<'a> RccIOPeripheral<'a, GPIOB, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable_gpiob(self) -> RccIOPeripheral<'a, GPIOB, PeripheralEnabled> {
        unsafe {
            self.0.apb2enr.modify(|_, w| {w.iopben().enabled()});
            transmute(self)
        }
    }
}
impl<'a> RccIOPeripheral<'a, GPIOC, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable_gpioc(self) -> RccIOPeripheral<'a, GPIOC, PeripheralEnabled> {
        unsafe {
            self.0.apb2enr.modify(|_, w| {w.iopcen().enabled()});
            transmute(self)
        }
    }
}

pub struct RccSPIPeripheral<'a, P: SPI, S : PeripheralState>(pub &'a RCC, PhantomData<(P, S)>);

impl<'a> RccSPIPeripheral<'a, SPI1, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable_spi1(self) -> RccSPIPeripheral<'a, SPI1, PeripheralEnabled> {
        unsafe {
            self.0.apb2enr.modify(|_, w| {w.spi1en().enabled()});
            transmute(self)
        }
    }
}

impl<'a> RccSPIPeripheral<'a, SPI2, PeripheralDisabled> {
    #[inline(always)]
    pub fn enable_spi2(self) -> RccSPIPeripheral<'a, SPI2, PeripheralEnabled> {
        unsafe {
            self.0.apb1enr.modify(|_, w| {w.spi2en().enabled()});
            transmute(self)
        }
    }
}


pub struct RccPeripherals<'a> {
    pub i2c1 : RccI2CPeripheral<'a, I2C1, PeripheralDisabled>,
    pub i2c2 : RccI2CPeripheral<'a, I2C2, PeripheralDisabled>,
    pub usart1 : RccUSARTPeripheral<'a, USART1, PeripheralDisabled>,
    pub usart2 : RccUSARTPeripheral<'a, USART2, PeripheralDisabled>,
    pub spi1 : RccSPIPeripheral<'a, SPI1, PeripheralDisabled>,
    pub spi2 : RccSPIPeripheral<'a, SPI2, PeripheralDisabled>,
    pub afio : RccAFIOPeripheral<'a, PeripheralDisabled>,
    pub tim2 : RccTIMPeripheral<'a, TIM2, PeripheralDisabled>,
    pub iopa : RccIOPeripheral<'a, GPIOA, PeripheralDisabled>,
    pub iopb : RccIOPeripheral<'a, GPIOB, PeripheralDisabled>,
    pub iopc : RccIOPeripheral<'a, GPIOC, PeripheralDisabled>,
}

pub struct Rcc<'a>(pub &'a RCC);

impl<'a> Rcc<'a> {
    #[inline(always)]
    pub fn get_peripherals(self) -> RccPeripherals<'a> {
        let rcc = self.0;
        RccPeripherals {
            i2c1 : RccI2CPeripheral(rcc, PhantomData),
            i2c2 : RccI2CPeripheral(rcc, PhantomData),
            usart1 : RccUSARTPeripheral(rcc, PhantomData),
            usart2 : RccUSARTPeripheral(rcc, PhantomData),
            spi1 : RccSPIPeripheral(rcc, PhantomData),
            spi2 : RccSPIPeripheral(rcc, PhantomData),
            afio : RccAFIOPeripheral(rcc, PhantomData),
            tim2 : RccTIMPeripheral(rcc, PhantomData),
            iopa : RccIOPeripheral(rcc, PhantomData),
            iopb : RccIOPeripheral(rcc, PhantomData),
            iopc : RccIOPeripheral(rcc, PhantomData),
        }
    }
}