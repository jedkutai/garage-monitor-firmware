//! Blink the built-in LED and send UART log messages.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use core::fmt::Write;

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{self as hal, rcc::Config};

use crate::hal::{pac, prelude::*};

mod constants;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Run the system clock at 48 MHz using the internal oscillator.
        let mut rcc = dp.RCC.freeze(Config::hsi().sysclk(48.MHz()));

        // Built-in LED on pin C13.
        let gpioc = dp.GPIOC.split(&mut rcc);
        let mut led = gpioc.pc13.into_push_pull_output();

        // UART transmit pin.
        let gpioa = dp.GPIOA.split(&mut rcc);
        let tx_pin = gpioa.pa9;

        // Configure USART1 at 115200 baud.
        let mut uart = dp
            .USART1
            .tx(tx_pin, 115_200.bps(), &mut rcc)
            .unwrap();

        // Delay using the Cortex-M SysTick timer.
        let mut delay = cp.SYST.delay(&rcc.clocks);

        let mut heartbeat: u32 = 0;

        writeln!(uart, "Garage monitor starting...\r").ok();

        loop {
            led.toggle();

            writeln!(uart, "heartbeat={heartbeat}\r").ok();
            heartbeat = heartbeat.wrapping_add(1);

            delay.delay_ms(1000);
        }
    }

    loop {}
}