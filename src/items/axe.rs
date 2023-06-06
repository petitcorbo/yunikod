use tui::{text::Span, style::{Style, Color}};
use crate::entities::{EntityKind, swing::Swing};

use super::Item;
use locales::t;

pub struct Axe {
    quantity: i8,
}

impl Axe {
    pub fn new() -> Self {
        Self {
            quantity: 1
        }
    }
}

impl Item for Axe {
    fn utilize(&self, coords: (i64, i64, crate::entities::Direction)) -> Option<crate::entities::EntityKind> {
        Some(EntityKind::Swing(Swing::new(coords.0, coords.1, coords.2, 10)))
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled("P", Style::default().fg(Color::White))
    }

    fn name(lang: String) -> String {
        t!("game.items.axe",lang)
    }

    fn damage(&self) -> u8 {
        2
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
