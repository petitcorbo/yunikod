use tui::{text::Span, style::Style};
use crate::items::{ItemKind, wood::Wood};
use super::{Block, BlockKind};

pub struct Tree {
    life: u8,
}

impl Block for Tree {
    fn generate() -> BlockKind {
        BlockKind::Tree(
            Tree {
                life: 10
            }
        )
    }

    fn shape<'a>(&self) -> tui::text::Span<'a> {
        Span::styled("⇞", Style::default().fg(tui::style::Color::Green))
    }

    fn collect(&mut self) -> ItemKind {
        self.life -= 1;
        ItemKind::Wood(Wood::new(2))
    }

    fn is_compatible_tool(item: &ItemKind) -> bool {
        match item {
            ItemKind::Axe(_) => true,
            _ => false
        }
    }

    fn is_destroyed(&self) -> bool {
        self.life == 0
    }
}
