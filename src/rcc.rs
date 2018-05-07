

#[allow(unused_imports)]
use common;

use ::core::marker::PhantomData;

use stm32f103xx::{RCC, GPIOA, GPIOB, GPIOC, GPIOD, I2C1, I2C2, TIM2, SPI1, SPI2, USART1, USART2, AFIO};

type_states!(IsEnabled, (Disabled, Enabled));

pub struct RccPeripheral<MOD, S>(PhantomData<MOD>, PhantomData<S>);

macro_rules! rcc_macro {
    ($name:ident, $apbenr:ident, $enable:ident, $apbrstr:ident, $reset:ident) => {

        impl RccPeripheral<$name,Disabled> {
            #[inline(always)]
            pub fn enable(self) -> RccPeripheral<$name, Enabled> {
                unsafe { (*RCC::ptr()).$apbenr.modify(|_, w| w.$enable().enabled()); }
                RccPeripheral(self.0, PhantomData)
            }

            #[inline(always)]
            pub fn reset(self) -> Self {
                unsafe {
                    (*RCC::ptr()).$apbrstr.modify(|_, w| w.$reset().set_bit());
                    (*RCC::ptr()).$apbrstr.modify(|_, w| w.$reset().clear_bit());
                }
                self
            }
        }
        
    };
}

rcc_macro!(USART1, apb2enr, usart1en, apb2rstr, usart1rst);
rcc_macro!(USART2, apb1enr, usart2en, apb1rstr, usart2rst);
rcc_macro!(TIM2, apb1enr, tim2en, apb1rstr, tim2rst);
rcc_macro!(I2C1, apb1enr, i2c1en, apb1rstr, i2c1rst);
rcc_macro!(I2C2, apb1enr, i2c2en, apb1rstr, i2c2rst);
rcc_macro!(AFIO, apb2enr, afioen, apb2rstr, afiorst);
rcc_macro!(GPIOA, apb2enr, iopaen, apb2rstr, ioparst);
rcc_macro!(GPIOB, apb2enr, iopben, apb2rstr, iopbrst);
rcc_macro!(GPIOC, apb2enr, iopcen, apb2rstr, iopcrst);
rcc_macro!(GPIOD, apb2enr, iopden, apb2rstr, iopdrst);
rcc_macro!(SPI1, apb2enr, spi1en, apb2rstr, spi1rst);
rcc_macro!(SPI2, apb1enr, spi2en, apb1rstr, spi2rst);


macro_rules! peripherals {
    ( $(($periph:ident, $name:ident)),* ) => {
        pub struct RccPeripherals {
            $( 
                pub $name : RccPeripheral<$periph, Disabled>,
            )*
        }

        pub struct Rcc<'a>(pub &'a RCC);

        impl <'a> Rcc<'a> {
            #[inline(always)]
            pub fn get_peripherals(self) -> RccPeripherals {
                RccPeripherals {
                    $(
                        $name: RccPeripheral(PhantomData, PhantomData),
                    )*
                }
            }
        }
    }
}

peripherals!{
    (USART1, usart1),
    (USART2, usart2),
    (TIM2, tim2),
    (I2C1, i2c1),
    (I2C2, i2c2),
    (AFIO, afio),
    (GPIOA, iopa),
    (GPIOB, iopb),
    (GPIOC, iopc),
    (GPIOD, iopd),
    (SPI1, spi1),
    (SPI2, spi2)
}