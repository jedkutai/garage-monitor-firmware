//! Blink the built-in LED using the SysTick timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{self as hal, rcc::Config};

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Run the system clock at 48 MHz using the internal oscillator.
        let mut rcc = dp.RCC.freeze(
            Config::hsi().sysclk(48.MHz())
        );

        // The Black Pill's built-in LED is connected to PC13.
        let gpioc = dp.GPIOC.split(&mut rcc);
        let mut led = gpioc.pc13.into_push_pull_output();

        // Create a blocking delay using the Cortex-M SysTick timer.
        let mut delay = cp.SYST.delay(&rcc.clocks);

        loop {
            led.toggle();
            delay.delay_ms(1000);
        }
    }

    loop {}
}