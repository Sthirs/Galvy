use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use crate::ui::display::Display;

use crate::ui::temperature_humidity_page::TemperatureHumidityPage;

#[derive(Debug, Copy, Clone)]
pub struct TemperatureHumidityData {
    pub temperature: f32,
    pub humidity: f32
}

pub struct UiController<D: Display> {
    rx_channel: Receiver<TemperatureHumidityData>,
    display: D
}

impl<D: Display> UiController<D> {

    pub fn new(rx_channel: Receiver<TemperatureHumidityData>, display: D) -> Self {
        UiController {
            rx_channel,
            display
        }
    }

    pub fn run(&mut self) {
        println!("Starting UiController...");

        let loop_sleep = Duration::from_secs(1);

        let mut temperature_humidity_page = TemperatureHumidityPage::new(self.display.size());

        loop {

            // Fetch data
            if let Some(data) = self.read_temperature_humidity_messages() {
                temperature_humidity_page.set_temperature(data.temperature);
                temperature_humidity_page.set_humidity(data.humidity);
            }

            // Draw page
            temperature_humidity_page.draw(&mut self.display);

            thread::sleep(loop_sleep);
        }
    }

    fn read_temperature_humidity_messages(&self) -> Option<TemperatureHumidityData> {
        if let Ok(data) = self.rx_channel.try_recv(){
            println!("Got: {} {}", data.temperature, data.humidity);
            Some(data)
        } else {
            None
        }
    }
}
