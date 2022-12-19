use crate::{
    items::Item,
    entities::{
        EntityKind,
        bullet::Bullet, Direction,
    }
};
use tui::{text::Span, style::{Style, Color}};

pub struct Gun;

impl Item for Gun {
    fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind> {
        let (x, y, direction) = coords;
        Some(EntityKind::Bullet(Bullet::new(x, y, direction)))
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled(" ", Style::default().fg(Color::Red))
    }
}
