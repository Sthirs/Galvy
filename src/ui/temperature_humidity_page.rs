
use embedded_graphics::geometry::Size;
use crate::ui::ui_element::UIElement;
use crate::ui::display::Display;
use crate::ui::divider_element::DividerElement;
use crate::ui::humidity_element::HumidityElement;
use crate::ui::temperature_element::TemperatureElement;


pub struct TemperatureHumidityPage
{
    temperature_element: TemperatureElement,
    divider_element: DividerElement,
    humidity_element: HumidityElement,
}

impl TemperatureHumidityPage {
    pub fn new(size: Size ) -> TemperatureHumidityPage {
        TemperatureHumidityPage {
            temperature_element: TemperatureElement::new(size),
            divider_element: DividerElement::new(size),
            humidity_element: HumidityElement::new(size)
        }
    }

    pub fn set_temperature(&mut self, temperature: f32) {
        // let temperature_element = &mut self.temperature_element;
        self.temperature_element.set_temperature(temperature);
    }

    pub fn set_humidity(&mut self, humidity: f32) {
        self.humidity_element.set_humidity(humidity);
    }

    pub fn draw<D: Display>(&mut self, display: &mut D) {

        display.clear();

        self.temperature_element.draw(display);
        self.divider_element.draw(display);
        self.humidity_element.draw(display);

        display.flush();
    }
}