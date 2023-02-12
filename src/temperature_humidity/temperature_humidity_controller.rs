use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use crate::temperature_humidity::temperature_humidity_sensor::{TemperatureHumidityData, TemperatureHumiditySensor};

pub struct TemperatureHumidityController<T: TemperatureHumiditySensor>  {
    temperature_humidity: TemperatureHumidityData,
    tx_channel: Sender<TemperatureHumidityData>,
    sensor: T,
}

impl<T: TemperatureHumiditySensor> TemperatureHumidityController<T> {

    pub fn new(tx_channel: Sender<TemperatureHumidityData>, sensor: T) -> Self {
        TemperatureHumidityController {
            temperature_humidity: TemperatureHumidityData {
                temperature: 0 as f32,
                humidity: 0 as f32,
            },
            tx_channel,
            sensor,
        }
    }

    pub fn run(mut self) {
        println!("Starting TemperatureHumidityController...");

        let loop_sleep = Duration::from_secs(3);

        loop {
            // Read temp and humidity
            if let Some(data) = self.sensor.read() {
                println!("{}°, {}% RH", data.temperature, data.humidity);
                self.temperature_humidity = data;
            } else {
                println!("No data returned from sensor")
            }

            // Send temperature and humidity to the UI
            if self.tx_channel.send(self.temperature_humidity).is_err() {
                println!("Error: fail to send message");
            }

            thread::sleep(loop_sleep);
        }
    }
}