use super::Item;
use tui::{text::Span, style::{Style, Color}};
use rust_i18n::t;
rust_i18n::i18n!("locales");

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
    fn utilize(&self, _coords: (i64, i64, crate::entities::Direction)) -> Option<crate::entities::EntityKind> {
        None
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled("°", Style::default().fg(Color::DarkGray))
    }

    fn name(lang: String) -> String {
        rust_i18n::set_locale(&lang); //set language
        t!("game.res.stone")
    }

    fn damage(&self) -> u8 {
        2
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
