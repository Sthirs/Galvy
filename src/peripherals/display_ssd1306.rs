use embedded_graphics::Drawable;
use embedded_graphics::geometry::{OriginDimensions, Point, Size};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::text::{Baseline, Text};
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::i2c::{I2C0, I2cDriver};
use ssd1306::mode::{BufferedGraphicsMode, DisplayConfig};
use ssd1306::prelude::{DisplayRotation, I2CInterface};
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};
use crate::ui::display::Display;

pub struct DisplaySsd1306 {
    display: Ssd1306<I2CInterface<I2cDriver<'static>>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>
}

impl DisplaySsd1306 {
    pub fn new(i2c: I2C0, sda: AnyIOPin, scl: AnyIOPin) -> Self {
        let i2c_driver = I2cDriver::new(i2c, sda, scl, &Default::default()).unwrap();
        let i2c_interface = I2CDisplayInterface::new(i2c_driver);
        let mut display = Ssd1306::new(
            i2c_interface,
            DisplaySize128x64,
            DisplayRotation::Rotate0,
        ).into_buffered_graphics_mode();

        display.init().unwrap();
        display.flush().unwrap();

        DisplaySsd1306 {
            display
        }
    }
}

impl Display for DisplaySsd1306 {

    fn write_string(&mut self, str: String, style: MonoTextStyle<BinaryColor>, position: Point) {
        Text::with_baseline(&str, position, style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
    }

    fn line(&mut self, style: PrimitiveStyle<BinaryColor>, start: Point, end: Point) {
        Line::new(start, end)
            .into_styled(style)
            .draw(&mut self.display).unwrap();
    }

    fn clear(&mut self) {
        self.display.clear();
    }

    fn flush(&mut self) {
        self.display.flush().unwrap();
    }

    fn size(&mut self) -> Size {
        self.display.size()
    }
}
