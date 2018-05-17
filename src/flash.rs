//! Flash memory

use stm32f103xx::{flash, FLASH};


#[allow(dead_code)]
pub struct Flash {
    flash: FLASH,
    pub acr: ACR,
}

impl Flash {
    pub fn new(flash: FLASH) -> Self {
        Self {
            flash: flash,
            acr: ACR {_0: ()}
        }
    }
}

/// Opaque ACR register
pub struct ACR {
    _0: (),
}

impl ACR {
    pub(crate) fn acr(&mut self) -> &flash::ACR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).acr }
    }
}
