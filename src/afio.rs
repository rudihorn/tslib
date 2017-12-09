
#[allow(unused_imports)]
#[macro_use]
use common;

use blue_pill::stm32f103xx::{AFIO, SPI1, SPI2};

use core::mem::transmute;
use core::marker::PhantomData;

use spi::SPI;

type_states!(IsEnabled, (NotEnabled, Enabled));
type_states!(IsRemapped, (NotConfigured, NotRemapped, Remapped));
type_group!(RemappedConfigurred, (NotRemapped, Remapped));

pub struct AfioI2C1Peripheral<'a, R>(pub &'a AFIO, PhantomData<(R)>)
where R: IsRemapped;

impl<'a> AfioI2C1Peripheral<'a, NotConfigured> {
    #[inline(always)]
    pub fn set_not_remapped(self) -> AfioI2C1Peripheral<'a, NotRemapped> {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn set_remapped(self) -> AfioI2C1Peripheral<'a, Remapped> {
        unsafe {
            self.0.mapr.modify(|_, w| { w.i2c1_remap().set_bit() });
            transmute(self) 
        }
    }
}

pub struct AfioSPIPeripheral<'a, S : SPI, R : IsRemapped>(pub &'a AFIO, PhantomData<(S, R)>);

impl<'a> AfioSPIPeripheral<'a, SPI1, NotConfigured> {
    #[inline(always)]
    pub fn set_not_remapped_spi1(self) -> AfioSPIPeripheral<'a, SPI1, NotRemapped> {
        unsafe {
            transmute(self)
        }
    }

    #[inline(always)]
    pub fn set_remapped_spi1(self) -> AfioSPIPeripheral<'a, SPI1, Remapped> {
        unsafe {
            self.0.mapr.modify(|_, w| {w.spi1_remap().set_bit()});
            transmute(self)
        }
    }
}

impl<'a> AfioSPIPeripheral<'a, SPI2, NotConfigured> {
    #[inline(always)]
    pub fn set_not_remapped_spi2(self) -> AfioSPIPeripheral<'a, SPI2, NotRemapped> {
        unsafe {
            transmute(self)
        }
    }

    /* #[inline(always)]
    pub fn set_remapped_spi2(self) -> AfioSPIPeripheral<'a, SPI2, Remapped> {
        unsafe {
            self.0.mapr.modify(|_, w| {w.spi2_remap().set_bit()});
            transmute(self)
        }
    } */
}

pub struct AfioPeripherals<'a> {
    pub i2c1 : AfioI2C1Peripheral<'a, NotConfigured>,
    pub spi1 : AfioSPIPeripheral<'a, SPI1, NotConfigured>,
}

pub struct Afio<'a>(pub &'a AFIO);

impl<'a> Afio<'a> {
    #[inline(always)]
    pub fn get_peripherals(self) -> AfioPeripherals<'a> {
        AfioPeripherals {
            i2c1: AfioI2C1Peripheral(self.0, PhantomData),
            spi1: AfioSPIPeripheral(self.0, PhantomData),
        }
    }
}