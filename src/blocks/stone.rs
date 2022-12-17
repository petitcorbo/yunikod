use tui::{text::Span, style::Style};
use crate::items::{ItemKind, wood::Wood};
use super::Block;

struct Tree {
    life: u32,
}

impl Block for Tree {
    fn shape<'a>(&self) -> tui::text::Span<'a> {
        Span::styled("$", Style::default().fg(tui::style::Color::Green))
    }

    fn collect(&mut self) -> ItemKind {
        self.life -= 1;
        ItemKind::Wood(Wood::new(2))
    }
}
