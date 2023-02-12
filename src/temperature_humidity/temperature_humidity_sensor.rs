#[derive(Debug, Copy, Clone)]
pub struct TemperatureHumidityData {
    pub temperature: f32,
    pub humidity: f32
}

pub trait TemperatureHumiditySensor {
    fn read(&mut self) -> Option<TemperatureHumidityData>;
}