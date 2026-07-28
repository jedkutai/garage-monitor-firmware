#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    i2c::{Config as I2cConfig, I2c},
    usart::{Config as UartConfig, UartTx},
};
use embassy_time::Timer;
use panic_halt as _;

const BME280_ADDRESS: u8 = 0x76;
const BME280_CHIP_ID_REGISTER: u8 = 0xD0;
const EXPECTED_BME280_CHIP_ID: u8 = 0x60;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());

    // Built-in LED on pin C13.
    let mut led = Output::new(peripherals.PC13, Level::High, Speed::Low);

    // Configure UART transmission on pin A9.
    let mut uart_config = UartConfig::default();
    uart_config.baudrate = 115_200;

    let mut uart = UartTx::new_blocking(peripherals.USART1, peripherals.PA9, uart_config).unwrap();

    // Configure I²C1:
    // Pin B6 = SCL
    // Pin B7 = SDA
    let mut i2c = I2c::new_blocking(
        peripherals.I2C1,
        peripherals.PB6,
        peripherals.PB7,
        I2cConfig::default(),
    );

    uart.blocking_write(b"Garage monitor starting...\r\n")
        .unwrap();

    uart.blocking_write(b"Checking BME280...\r\n").unwrap();

    let mut chip_id = [0_u8; 1];

    match i2c.blocking_write_read(BME280_ADDRESS, &[BME280_CHIP_ID_REGISTER], &mut chip_id) {
        Ok(()) if chip_id[0] == EXPECTED_BME280_CHIP_ID => {
            uart.blocking_write(b"BME280 confirmed: chip ID 0x60\r\n")
                .unwrap();
        }

        Ok(()) => {
            uart.blocking_write(b"Unexpected BME280 chip ID\r\n")
                .unwrap();
        }

        Err(_) => {
            uart.blocking_write(b"Failed to communicate with BME280\r\n")
                .unwrap();
        }
    }

    loop {
        led.set_low();

        uart.blocking_write(b"heartbeat\r\n").unwrap();

        Timer::after_millis(500).await;

        led.set_high();

        Timer::after_millis(500).await;
    }
}
