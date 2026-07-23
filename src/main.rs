//! Test UART and communicate with the BME280 over I²C.

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

const BME280_ADDRESS: u8 = 0x76;
const BME280_CHIP_ID_REGISTER: u8 = 0xD0;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Run the system clock at 48 MHz using the internal oscillator.
        let mut rcc = dp.RCC.freeze(Config::hsi().sysclk(48.MHz()));

        // Configure the built-in LED on pin C13.
        let gpioc = dp.GPIOC.split(&mut rcc);
        let mut led = gpioc.pc13.into_push_pull_output();

        // Configure UART transmission on pin A9.
        let gpioa = dp.GPIOA.split(&mut rcc);
        let tx_pin = gpioa.pa9;

        let mut uart = dp
            .USART1
            .tx(tx_pin, 115_200.bps(), &mut rcc)
            .unwrap();

        // Configure I²C1 using:
        // Pin B6 = SCL
        // Pin B7 = SDA
        let gpiob = dp.GPIOB.split(&mut rcc);
        let scl = gpiob.pb6;
        let sda = gpiob.pb7;

        let mut i2c = dp.I2C1.i2c((scl, sda), 100.kHz(), &mut rcc);

        // Create a blocking delay using the SysTick timer.
        let mut delay = cp.SYST.delay(&rcc.clocks);

        writeln!(uart, "Garage monitor starting...\r").ok();

        let mut chip_id = [0_u8; 1];

        match i2c.write_read(
            BME280_ADDRESS,
            &[BME280_CHIP_ID_REGISTER],
            &mut chip_id,
        ) {
            Ok(()) => {
                writeln!(uart, "BME280 chip ID: 0x{:02X}\r", chip_id[0]).ok();
            }
            Err(_) => {
                writeln!(uart, "Failed to communicate with BME280\r").ok();
            }
        }

        loop {
            led.toggle();
            delay.delay_ms(1000);
        }
    }

    loop {}
}