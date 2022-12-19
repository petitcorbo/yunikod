use tui::{text::Span, style::{Style, Color}};
use super::Item;

pub struct Axe {
    quantity: u32,
}

impl Axe {
    pub fn new(quantity: u32) -> Self {
        Self {
            quantity
        }
    }
}

impl Item for Axe {
    fn utilize(&self, coords: (f64, f64, crate::entities::Direction)) -> Option<crate::entities::EntityKind> {
        None
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled(" ", Style::default().fg(Color::Red))
    }
}
