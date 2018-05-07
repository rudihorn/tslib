
#[allow(unused_imports)]
use common;

use stm32f103xx::{AFIO};

use core::marker::PhantomData;

type_states!(IsRemapped, (NotConfigured, NotRemapped, Remapped));

macro_rules! peripheral_macro  {
    ($name:ident, $remap_bit:ident) => {
        pub struct $name<'a, R>(pub &'a AFIO, PhantomData<R>)
        where R: IsRemapped;

        impl <'a> $name<'a, NotConfigured> {
            #[inline(always)]
            pub fn set_not_remapped(self) -> $name<'a, NotRemapped> {
                $name(self.0, PhantomData)
            }

            #[inline(always)]
            pub fn set_remapped(self) -> $name<'a, Remapped> {
                self.0.mapr.modify(|_, w| { w.$remap_bit().set_bit() });
                $name(self.0, PhantomData)
            }
        }
    };
}

peripheral_macro!(AfioUSART1Peripheral, usart1_remap);
peripheral_macro!(AfioUSART2Peripheral, usart2_remap);
peripheral_macro!(AfioI2C1Peripheral, i2c1_remap);
peripheral_macro!(AfioSPI1Peripheral, spi1_remap);

macro_rules! peripherals {
    ( $(($periph:ident, $name:ident)),* ) => {
        pub struct AfioPeripherals<'a> {
            $( 
                pub $name : $periph<'a, NotConfigured>,
            )*
        }

        pub struct Afio<'a>(pub &'a AFIO);

        impl <'a> Afio<'a> {
            #[inline(always)]
            pub fn get_peripherals(self) -> AfioPeripherals<'a> {
                AfioPeripherals {
                    $(
                        $name: $periph(self.0, PhantomData),
                    )*
                }
            }
        }
    }
}

peripherals!(
    (AfioUSART1Peripheral, usart1),
    (AfioUSART2Peripheral, usart2),
    (AfioI2C1Peripheral, i2c1),
    (AfioSPI1Peripheral, spi1_remap)
);