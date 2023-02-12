use embedded_graphics::geometry::Size;
use embedded_graphics::mono_font::iso_8859_15::FONT_4X6;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::mono_font::iso_8859_5::FONT_6X12;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;

use crate::ui::display::Display;
use crate::ui::ui_element::UIElement;

#[derive(Clone, Copy)]
pub struct TemperatureElement {
    temperature: f32,
    display_size: Size
}

impl UIElement for TemperatureElement {
    fn draw<D: Display>(&self, display: &mut D) {

        let temp_integer_str = self.get_int_part_as_string();
        let temp_decimal_str = self.get_decimal_part_as_string();
        println!("Draw temperature {}{}", temp_integer_str, temp_decimal_str);

        let base_line_temp = 5;
        let mut text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::On)
            .build();

        // Draw integer part
        let temp_integer_char_count: u32 = temp_integer_str.chars().count() as u32;
        let position_x = (self.display_size.width / 2 - temp_integer_char_count * text_style.font.character_size.width) as i32 + 5;
        display.write_string(temp_integer_str, text_style, Point::new(position_x, base_line_temp));

        // Draw decimal part
        let position_x = (self.display_size.width / 2) as i32 + 5;
        text_style.font = &FONT_6X12;
        display.write_string(temp_decimal_str, text_style, Point::new(position_x, base_line_temp + (20 - text_style.font.character_size.height - 2) as i32));

        // Draw unit
        text_style.font = &FONT_4X6;
        display.write_string(String::from("C"), text_style, Point::new(position_x + 8, base_line_temp + 1));
        display.write_string(String::from("°"), text_style, Point::new(position_x + 4, base_line_temp + 1));
    }
}

impl TemperatureElement {
    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }

    pub fn new(display_size: Size) -> Self {
        TemperatureElement {
            temperature: 0 as f32,
            display_size
        }
    }

    fn get_int_part_as_string(& self) -> String {
        let int_part = self.temperature as i32;
        format!("{}", int_part)
    }

    fn get_decimal_part_as_string(& self) -> String {
        let decimal_part = (self.temperature * 10.0) as i32 % 10;
        format!(".{}", decimal_part)
    }
}