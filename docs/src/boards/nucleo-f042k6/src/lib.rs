#![no_std]
#![feature(const_fn)]
#![allow(non_camel_case_types)]

pub extern crate stm32f042_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;

pub use cortex_m::*;
pub use cortex_m_rt::*;
pub use hal::stm32::interrupt::*;
pub use hal::stm32::*;
pub use hal::*;
