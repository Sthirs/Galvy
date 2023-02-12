use embedded_graphics::geometry::Size;




use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use embedded_graphics::primitives::PrimitiveStyleBuilder;

use crate::ui::display::Display;
use crate::ui::ui_element::UIElement;

#[derive(Clone, Copy)]
pub struct DividerElement {
    display_size: Size
}

impl UIElement for DividerElement {
    fn draw<D: Display>(&self, display: &mut D) {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .build();
        display.line(style, Point::new((self.display_size.width / 2 - 30) as i32, (self.display_size.height / 2) as i32), Point::new((self.display_size.width / 2 + 30) as i32, (self.display_size.height / 2) as i32));
    }
}

impl DividerElement {
    pub fn new(display_size: Size) -> Self {
        DividerElement {
            display_size
        }
    }
}