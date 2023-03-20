use crate::{
    items::Item,
    entities::{
        EntityKind,
        fire::Fire, Direction,
    }
};
use tui::{text::Span, style::{Style, Color}};

pub struct DragonSoul {
    quantity: i8
}

impl Item for DragonSoul {
    fn utilize(&self, coords: (i64, i64, Direction)) -> Option<EntityKind> {
        let (x, y, direction) = coords;
        Some(EntityKind::Fire(Fire::new(x, y, direction)))
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled("@", Style::default().fg(Color::Red))
    }

    fn name<'a>() -> &'a str {
        "dragon soul"
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
