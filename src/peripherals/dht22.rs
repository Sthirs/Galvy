use dht_sensor::{dht22, DhtReading};
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{AnyIOPin, InputOutput, PinDriver};
use crate::temperature_humidity::temperature_humidity_sensor::{TemperatureHumidityData, TemperatureHumiditySensor};

pub struct DHT22<'d> {
    pin: PinDriver<'d, AnyIOPin, InputOutput>,
}

impl TemperatureHumiditySensor for DHT22<'_> {

    fn read(&mut self) -> Option<TemperatureHumidityData> {
        match dht22::Reading::read(&mut Ets{}, &mut self.pin) {
            Ok(dht22::Reading {
                   temperature,
                   relative_humidity,
               }
            ) => Some(
                TemperatureHumidityData {
                    temperature,
                    humidity: relative_humidity
                }
            ),
            Err(_e) => None,
        }
    }
}

impl DHT22<'_> {
    pub fn new(gpio: AnyIOPin) -> Self {
        let mut dht22_gpio = PinDriver::input_output_od(gpio).unwrap();
        dht22_gpio.set_high().ok();

        DHT22 { pin: dht22_gpio }
    }
}
