use super::Item;
use tui::{text::Span, style::{Style, Color}};
use locales::t;

pub struct Hand {
    quantity: i8,
}

impl Hand {
    pub fn new(quantity: i8) -> Self {
        Self {
            quantity
        }
    }
}

impl Item for Hand {
    fn utilize(&self, _coords: (i64, i64, crate::entities::Direction)) -> Option<crate::entities::EntityKind> {
        None
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled("h", Style::default().fg(Color::Red))
    }

    fn name(lang: String) -> String {
      t!("game.entity.player.hand",lang)
    }

    fn damage(&self) -> u8 {
        1
    }

    fn quantity(&self) -> i8 {
        self.quantity
    }

    fn max_quantity(&self) -> i8 {
        1
    }

    fn change_quantity(&mut self, amount: i8) -> i8 {
        if self.quantity == self.max_quantity() {
            return amount;
        }
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
