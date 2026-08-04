#![no_std]
#![no_main]

use bme280_rs::{AsyncBme280, Configuration, Oversampling, SensorMode};
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, dma,
    gpio::{Level, Output, Speed},
    i2c::{self, Config as I2cConfig, I2c},
    peripherals,
    usart::{Config as UartConfig, UartTx},
};
use embassy_time::{Delay, Timer};
use panic_halt as _;

use crate::{
    components::sensor_sample::SensorSample,
    constants::setup::{BAUDRATE, BME280_ADDRESS},
};

mod components;
mod constants;

// Connect I²C1 and its DMA channels to their interrupt handlers.
bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;

    DMA1_STREAM6 => dma::InterruptHandler<peripherals::DMA1_CH6>;
    DMA1_STREAM0 => dma::InterruptHandler<peripherals::DMA1_CH0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());

    // Built-in LED on pin C13.
    let mut led = Output::new(peripherals.PC13, Level::High, Speed::Low);

    // Configure UART transmission through pin A9.
    let mut uart_config = UartConfig::default();
    uart_config.baudrate = BAUDRATE;

    let mut uart = UartTx::new_blocking(peripherals.USART1, peripherals.PA9, uart_config).unwrap();

    uart.blocking_write(b"Garage monitor starting...\r\n")
        .unwrap();

    // Configure asynchronous I²C1:
    //
    // Pin B6 = SCL
    // Pin B7 = SDA
    // DMA1_CH6 = transmit DMA
    // DMA1_CH0 = receive DMA
    let i2c = I2c::new(
        peripherals.I2C1,
        peripherals.PB6,
        peripherals.PB7,
        peripherals.DMA1_CH6,
        peripherals.DMA1_CH0,
        Irqs,
        I2cConfig::default(),
    );

    // Create the async BME280 driver.
    //
    // SDO is connected to GND, so the sensor address is 0x76.
    let mut sensor = AsyncBme280::new_with_address(i2c, BME280_ADDRESS, Delay);

    uart.blocking_write(b"Initializing BME280...\r\n").unwrap();

    // Reset the sensor and load its calibration values.
    if sensor.init().await.is_err() {
        uart.blocking_write(b"Failed to initialize BME280\r\n")
            .unwrap();

        // Blink rapidly to indicate an initialization error.
        loop {
            led.set_low();
            Timer::after_millis(100).await;

            led.set_high();
            Timer::after_millis(100).await;
        }
    }

    // Enable temperature, pressure, and humidity measurements.
    let sensor_config = Configuration::default()
        .with_temperature_oversampling(Oversampling::Oversample1)
        .with_pressure_oversampling(Oversampling::Oversample1)
        .with_humidity_oversampling(Oversampling::Oversample1)
        .with_sensor_mode(SensorMode::Normal);

    if sensor
        .set_sampling_configuration(sensor_config)
        .await
        .is_err()
    {
        uart.blocking_write(b"Failed to configure BME280\r\n")
            .unwrap();

        // Blink rapidly to indicate a configuration error.
        loop {
            led.set_low();
            Timer::after_millis(100).await;

            led.set_high();
            Timer::after_millis(100).await;
        }
    }

    // Allow time for the first measurement.
    Timer::after_millis(100).await;

    uart.blocking_write(b"BME280 initialized successfully.\r\n")
        .unwrap();

    let mut sensor_sample = SensorSample::new();
    loop {
        // Turn on the LED while reading the sensor.
        led.set_low();

        match sensor.read_sample().await {
            Ok(sample) => {
                sensor_sample.update_and_log(sample, &mut uart);
            }

            Err(_) => {
                uart.blocking_write(b"Failed to read BME280 measurements\r\n")
                    .unwrap();
            }
        }

        Timer::after_millis(500).await;
        // Turn the LED off after the reading.
        led.set_high();

        // Wait two seconds without blocking Embassy's executor.
        Timer::after_millis(2000).await;
    }
}

