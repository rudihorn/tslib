
#[allow(unused_imports)]
use common;

use core::any::Any;
use core::fmt::{Display, Formatter, Result};
use core::ops::Deref;
use core::ptr;
use core::marker::PhantomData;
use core::mem::transmute;

use gpio::{Input, PinOutput, GpioPin, Pin9, Pin10, PinMode, PinCnf1, PinCnf2};
use afio::{AfioSPIPeripheral, NotRemapped};
use stm32f103xx::{GPIOA, USART1, usart1, gpioa};

/// SPI instance that can be used with the `Spi` abstraction
pub unsafe trait USART: Deref<Target = usart1::RegisterBlock> {
    /// GPIO block associated to this SPI instance
    type GPIO: Deref<Target = gpioa::RegisterBlock>;
}

unsafe impl USART for USART1 {
    type GPIO = GPIOA;
}

unsafe impl USART for USART2 {
    type GPIO = GPIOB;
}

