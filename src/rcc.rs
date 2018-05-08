#[allow(unused_imports)]
use common;

use core::cmp;
use core::marker::PhantomData;

use stm32f103xx::{rcc, RCC, GPIOA, GPIOB, GPIOC, GPIOD, I2C1, I2C2, TIM2, SPI1, SPI2, USART1, USART2, AFIO};

use time::Hertz;
use flash::ACR;

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

pub struct Rcc{
    pub cfgr: CFGR,
    pub peripherals: RccPeripherals,
}

impl Rcc {
    pub fn new(_r: RCC) -> Self {
        Rcc{
            cfgr: CFGR{
                hclk: None,
                pclk1: None,
                pclk2: None,
                sysclk: None,
            },
            peripherals: Rcc::get_peripherals()
        }
    }

    fn ptr<'a>() -> &'a rcc::RegisterBlock {
        unsafe { &(*RCC::ptr()) }
    }
}

macro_rules! peripherals {
    ( $(($periph:ident, $name:ident)),* ) => {
        pub struct RccPeripherals {
            $( 
                pub $name : RccPeripheral<$periph, Disabled>,
            )*
        }

        impl Rcc {
            #[inline(always)]
            fn get_peripherals() -> RccPeripherals {
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

// from stm32f103xx-hal

const HSI: u32 = 8_000_000; // Hz

pub struct CFGR {
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
}

impl CFGR {
    pub fn hclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.hclk = Some(freq.into().0);
        self
    }

    pub fn pclk1<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk1 = Some(freq.into().0);
        self
    }

    pub fn pclk2<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk2 = Some(freq.into().0);
        self
    }

    pub fn sysclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.sysclk = Some(freq.into().0);
        self
    }

    pub fn freeze(self, acr: &mut ACR) -> Clocks {
        // TODO ADC & USB clocks

        let pllmul = (4 * self.sysclk.unwrap_or(HSI) + HSI) / HSI / 2;
        let pllmul = cmp::min(cmp::max(pllmul, 2), 16);
        let pllmul_bits = if pllmul == 2 {
            None
        } else {
            Some(pllmul as u8 - 2)
        };

        let sysclk = pllmul * HSI / 2;

        assert!(sysclk < 72_000_000);

        let hpre_bits = self.hclk
            .map(|hclk| match sysclk / hclk {
                0 => unreachable!(),
                1 => 0b0111,
                2 => 0b1000,
                3...5 => 0b1001,
                6...11 => 0b1010,
                12...39 => 0b1011,
                40...95 => 0b1100,
                96...191 => 0b1101,
                192...383 => 0b1110,
                _ => 0b1111,
            })
            .unwrap_or(0b0111);

        let hclk = sysclk / (1 << (hpre_bits - 0b0111));

        assert!(hclk < 72_000_000);

        let ppre1_bits = self.pclk1
            .map(|pclk1| match hclk / pclk1 {
                0 => unreachable!(),
                1 => 0b011,
                2 => 0b100,
                3...5 => 0b101,
                6...11 => 0b110,
                _ => 0b111,
            })
            .unwrap_or(0b011);

        let ppre1 = 1 << (ppre1_bits - 0b011);
        let pclk1 = hclk / (ppre1 as u32);

        assert!(pclk1 <= 36_000_000);

        let ppre2_bits = self.pclk2
            .map(|pclk2| match hclk / pclk2 {
                0 => unreachable!(),
                1 => 0b011,
                2 => 0b100,
                3...5 => 0b101,
                6...11 => 0b110,
                _ => 0b111,
            })
            .unwrap_or(0b011);

        let ppre2 = 1 << (ppre2_bits - 0b011);
        let pclk2 = hclk / (ppre2 as u32);

        assert!(pclk2 < 72_000_000);

        // adjust flash wait states
        unsafe {
            acr.acr().write(|w| {
                w.latency().bits(if sysclk <= 24_000_000 {
                    0b000
                } else if sysclk <= 48_000_000 {
                    0b001
                } else {
                    0b010
                })
            })
        }

        let rcc = unsafe { &*RCC::ptr() };
        if let Some(pllmul_bits) = pllmul_bits {
            // use PLL as source

            rcc.cfgr.write(|w| unsafe { w.pllmul().bits(pllmul_bits) });

            rcc.cr.write(|w| w.pllon().enabled());

            while rcc.cr.read().pllrdy().is_unlocked() {}

            rcc.cfgr.modify(|_, w| unsafe {
                w.ppre2()
                    .bits(ppre2_bits)
                    .ppre1()
                    .bits(ppre1_bits)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    .pll()
            });
        } else {
            // use HSI as source

            rcc.cfgr.write(|w| unsafe {
                w.ppre2()
                    .bits(ppre2_bits)
                    .ppre1()
                    .bits(ppre1_bits)
                    .hpre()
                    .bits(hpre_bits)
                    .sw()
                    .hsi()
            });
        }

        Clocks {
            hclk: Hertz(hclk),
            pclk1: Hertz(pclk1),
            pclk2: Hertz(pclk2),
            ppre1,
            ppre2,
            sysclk: Hertz(sysclk),
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    hclk: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    ppre1: u8,
    ppre2: u8,
    sysclk: Hertz,
}

impl Clocks {
    /// Returns the frequency of the AHB
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the APB1
    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    /// Returns the frequency of the APB2
    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    pub(crate) fn ppre1(&self) -> u8 {
        self.ppre1
    }

    // TODO remove `allow`
    #[allow(dead_code)]
    pub(crate) fn ppre2(&self) -> u8 {
        self.ppre2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }
}
