use super::Item;
use tui::{text::Span, style::{Style, Color}};

pub struct Stone {
    quantity: i8,
}

impl Stone {
    pub fn new(quantity: i8) -> Self {
        Self {
            quantity
        }
    }
}

impl Item for Stone {
    fn utilize(&self, coords: (f64, f64, crate::entities::Direction)) -> Option<crate::entities::EntityKind> {
        None
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled("°", Style::default().fg(Color::DarkGray))
    }

    fn name<'a>() -> &'a str {
        "stone"
    }

    fn quantity(&self) -> i8 {
        self.quantity
    }

    fn max_quantity(&self) -> i8 {
        10
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
