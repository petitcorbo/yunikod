use tui::{text::Span, style::{Style, Color}};
use super::Item;

pub struct Axe {
    quantity: i8,
}

impl Axe {
    pub fn new(quantity: i8) -> Self {
        Self {
            quantity
        }
    }
}

impl Item for Axe {
    fn utilize(&self, _coords: (f64, f64, crate::entities::Direction)) -> Option<crate::entities::EntityKind> {
        None
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled("P", Style::default().fg(Color::White))
    }

    fn name<'a>() -> &'a str {
        "axe"
    }

    fn quantity(&self) -> i8 {
        self.quantity
    }

    fn max_quantity(&self) -> i8 {
        1
    }

    fn change_quantity(&mut self, amount: i8) -> i8 {
        let prevision = self.quantity + amount;
        if prevision < 0 {
            self.quantity = 0;
            return prevision;
        } else if prevision > self.max_quantity() {
            self.quantity = self.max_quantity();
            return self.max_quantity() - prevision;
        } else {
            self.quantity = prevision;
            return 0;
        }
    }
}
