use super::Item;
use crate::entities::{EntityKind, swing::Swing};
use tui::{text::Span, style::{Style, Color}};
use locales::t;

pub struct Stick {
    quantity: i8,
}

impl Stick {
    pub fn new(quantity: i8) -> Self {
        Self {
            quantity
        }
    }
}

impl Item for Stick {
    fn utilize(&self, coords: (i64, i64, crate::entities::Direction)) -> Option<crate::entities::EntityKind> {
        Some(EntityKind::Swing(Swing::new(coords.0, coords.1, coords.2, 10)))
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        //Span::styled("t", Style::default().fg(Color::Red))
        Span::styled("ɻ", Style::default().fg(Color::Rgb(145, 77, 5)))
    }

    fn name(lang: String) -> String {
        t!("game.items.stick",lang)
    }

    fn damage(&self) -> u8 {
        2
    }

    fn quantity(&self) -> i8 {
        self.quantity
    }

    fn max_quantity(&self) -> i8 {
        20
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
