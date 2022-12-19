use crate::{
    items::Item,
    entities::{
        EntityKind,
        fire::Fire, Direction,
    }
};
use tui::{text::Span, style::{Style, Color}};

pub struct FlameThrower;

impl Item for FlameThrower {
    fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind> {
        let (x, y, direction) = coords;
        Some(EntityKind::Fire(Fire::new(x, y, direction)))
    }

    fn shape<'a>() -> tui::text::Span<'a> {
        Span::styled("@", Style::default().fg(Color::Red))
    }
}
