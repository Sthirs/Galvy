mod peripherals;
mod ui;
mod temperature_humidity;

use temperature_humidity::temperature_humidity_controller::*;
use ui::ui_controller::*;

extern crate core;

use std::sync::mpsc;
use std::thread;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _;
use crate::peripherals::dht22::DHT22;
use peripherals::display_ssd1306::DisplaySsd1306;
use temperature_humidity::temperature_humidity_sensor::TemperatureHumidityData;
use ui::ui_controller::TemperatureHumidityData as UITemperatureHumidityData;


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Init...");
    let peripherals = Peripherals::take().unwrap();
    let scl = AnyIOPin::from(peripherals.pins.gpio22);
    let sda = AnyIOPin::from(peripherals.pins.gpio21);
    let i2c = peripherals.i2c0;
    let gpio_dht22 = AnyIOPin::from(peripherals.pins.gpio19);

    let (ui_tx_channel, ui_rx_channel) = mpsc::channel::<UITemperatureHumidityData>();
    let (temp_hum_tx_channel, temp_hum_rx_channel) = mpsc::channel::<TemperatureHumidityData>();

    // Temperature-humidity thread
    thread::Builder::new()
        .stack_size(8 * 1024)
        .spawn(move || {
            let dht22 = DHT22::new(gpio_dht22);
            let temperature_humidity_sensor_controller = TemperatureHumidityController::new(temp_hum_tx_channel, dht22);
            temperature_humidity_sensor_controller.run();
        })
        .unwrap();

    // UI thread
    thread::Builder::new()
        .stack_size(8 * 1024)
        .spawn(move || {
            let display = DisplaySsd1306::new(i2c, sda, scl);
            let mut ui_controller = UiController::new(ui_rx_channel, display);
            ui_controller.run();
        })
        .unwrap();

    // Communication between UI and temp/hum sensor threads
    thread::Builder::new()
        .stack_size(8 * 1024)
        .spawn(move || {
            loop {
                if let Ok(data) = temp_hum_rx_channel.recv() {
                    let ui_temp_hum_data = UITemperatureHumidityData {
                        temperature: data.temperature,
                        humidity: data.humidity,
                    };
                    ui_tx_channel.send(ui_temp_hum_data).unwrap();
                }

            }
        })
        .unwrap();
}
