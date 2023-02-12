use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{Point, Size};
use embedded_graphics::primitives::PrimitiveStyle;

pub trait Display {
    fn write_string(&mut self, str: String, style: MonoTextStyle<BinaryColor>, position: Point);

    fn line(&mut self, style: PrimitiveStyle<BinaryColor>, start: Point, end: Point);

    fn clear(&mut self);

    fn flush(&mut self);

    fn size(&mut self) -> Size;
}