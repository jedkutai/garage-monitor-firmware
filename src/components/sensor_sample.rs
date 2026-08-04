use bme280_rs::Sample;
use embassy_stm32::{mode::Blocking, usart::UartTx};

#[derive(Debug, Default, Clone, Copy)]
pub struct SensorSample {
    temperature_celsius: f32,
    humidity: f32,
    pressure_pascals: f32,
}

impl SensorSample {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_and_log(&mut self, sample: Sample, uart: &mut UartTx<'_, Blocking>) {
        match (sample.temperature, sample.humidity, sample.pressure) {
            (Some(temperature), Some(humidity), Some(pressure)) => {
                // The sensor returns pressure in pascals.
                self.temperature_celsius = temperature;
                self.humidity = humidity;
                self.pressure_pascals = pressure;

                uart.blocking_write(b"Successful Update\r\n").unwrap();
            }

            _ => {
                uart.blocking_write(b"One or more BME280 measurements are disabled\r\n")
                    .unwrap();
            }
        }
    }

    // pub fn temperature_celsius(&self) -> f32 {
    //     self.temperature_celsius
    // }

    // pub fn temperature_fahrenheit(&self) -> f32 {
    //     self.temperature_celsius * 9.0 / 5.0 + 32.0
    // }


    // pub fn humidity(&self) -> f32 {
    //     self.humidity
    // }

    // pub fn pressure_hectopascals(&self) -> f32 {
    //     self.pressure_pascals / 100.0
    // }
}
