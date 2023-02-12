
use crate::ui::display::Display;

pub trait UIElement {
    fn draw<D: Display>(&self, display: &mut D);
}