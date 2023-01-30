use crate::{
    items::Item,
    entities::{
        EntityKind,
        bullet::Bullet, Direction,
    }
};
use tui::{text::Span, style::{Style, Color}};

pub struct OnyxHeart {
    quantity: i8
}

impl Item for OnyxHeart {
    fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind> {
        let (x, y, direction) = coords;
        Some(EntityKind::Bullet(Bullet::new(x, y, direction)))
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled(" ", Style::default().fg(Color::Red))
    }

    fn name<'a>() -> &'a str {
        "onyx heart"
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
