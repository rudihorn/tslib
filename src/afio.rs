
#[allow(unused_imports)]
use common;

use rcc;
use rcc::RccPeripheral;
use stm32f103xx::{AFIO, USART1, USART2, I2C1, SPI1};

use core::marker::PhantomData;

type_states!(IsRemapped, (NotConfigured, NotRemapped, Remapped));

pub struct AfioPeripheral<'a, P, R>(pub &'a AFIO, PhantomData<P>, PhantomData<R>)
where R: IsRemapped;

macro_rules! peripheral_macro  {
    ($periph:ident, $remap_bit:ident) => {

        impl <'a> AfioPeripheral<'a, $periph, NotConfigured> {
            #[inline(always)]
            pub fn set_not_remapped(self) -> AfioPeripheral<'a, $periph, NotRemapped> {
                AfioPeripheral(self.0, self.1, PhantomData)
            }

            #[inline(always)]
            pub fn set_remapped(self) -> AfioPeripheral<'a, $periph, Remapped> {
                self.0.mapr.modify(|_, w| { w.$remap_bit().set_bit() });
                AfioPeripheral(self.0, self.1, PhantomData)
            }
        }
    };
}

peripheral_macro!(USART1, usart1_remap);
peripheral_macro!(USART2, usart2_remap);
peripheral_macro!(I2C1, i2c1_remap);
peripheral_macro!(SPI1, spi1_remap);

pub struct Afio<'a>(pub &'a AFIO);

impl <'a> Afio<'a> {
    pub fn new(afio : &'a AFIO, _rcc_afio: RccPeripheral<AFIO, rcc::Enabled>) -> Self {
        Afio(afio)
    }
}

macro_rules! peripherals {
    ( $(($periph:ident, $name:ident)),* ) => {
        pub struct AfioPeripherals<'a> {
            $( 
                pub $name : AfioPeripheral<'a, $periph, NotConfigured>,
            )*
        }

        impl <'a> Afio<'a> {
            #[inline(always)]
            pub fn get_peripherals(self) -> AfioPeripherals<'a> {
                AfioPeripherals {
                    $(
                        $name: AfioPeripheral(self.0, PhantomData, PhantomData),
                    )*
                }
            }
        }
    }
}

peripherals!(
    (USART1, usart1),
    (USART2, usart2),
    (I2C1, i2c1),
    (SPI1, spi1)
);