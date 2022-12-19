use tui::{text::Span, style::Style};
use crate::items::{ItemKind, wood::Wood};
use super::{Block, BlockKind};

pub struct Stones {
    life: u32,
}

impl Stones {
    pub fn new() -> Self {
        Self {
            life: 1
        }
    }
}

impl Block for Stones {
    fn generate() -> BlockKind {
        BlockKind::Stones(
            Self {
                life: 15
            }
        )
    }

    fn shape<'a>(&self) -> tui::text::Span<'a> {
        Span::styled("⣿", Style::default().fg(tui::style::Color::Gray))
    }

    fn collect(&mut self) -> ItemKind {
        if self.life > 0 {
            self.life -= 1;
            ItemKind::Wood(Wood::new(2))
        } else {
            ItemKind::Wood(Wood::new(0))
        }
    }

    fn is_compatible_tool(item: ItemKind) -> bool {
        match item {
            ItemKind::Hand(_) => true,
            ItemKind::Pickaxe(_) => true,
            _ => false,
        }
    }

    fn is_destroyed(&self) -> bool {
        self.life == 0
    }
}
