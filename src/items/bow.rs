use crate::{
    items::Item,
    entities::{
        EntityKind,
        Direction,
        arrow::Arrow
    }
};
use tui::{text::Span, style::{Style, Color}};

pub struct Bow {
    quantity: i8
}

impl Bow {
    pub fn new() -> Bow {
        Bow {
            quantity: 1
        }
    }
}

impl Item for Bow {
    fn utilize(&self, coords: (i64, i64, Direction)) -> Option<EntityKind> {
        let (x, y, direction) = coords;
        Some(EntityKind::Arrow(Arrow::new(x, y, direction, 5)))
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled(")", Style::default().fg(Color::Red))
    }

    fn name<'a>() -> &'a str {
        "bow"
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
