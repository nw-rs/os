#![no_std]
#![no_main]
#![allow(dead_code)]

extern crate cortex_m_rt as rt;

use rt::entry;

use core::panic::PanicInfo;

use cortex_m_semihosting::hprintln;

pub const HSE: u32 = 8;
pub const PLL_M: u8 = 8;
pub const PLL_N: u16 = 384;
pub const PLL_P: u32 = 2;
pub const PLL_Q: u8 = 8;
pub const SYSCLK: u32 = 192_000_000;

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hprintln!("{:?}", info);
    loop {}
}

#[entry]
fn main() -> ! {
    hprintln!("entered external");
    loop {
        cortex_m::asm::nop();
    }
}
