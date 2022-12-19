use tui::{text::Span, style::{Style, Color}};
use crate::items::{ItemKind, wood::Wood};
use super::{Block, BlockKind};

pub struct IronOre {
    life: u32,
}

impl Block for IronOre {
    fn generate() -> BlockKind {
        BlockKind::IronOre(
            Self {
                life: 15
            }
        )
    }

    fn shape<'a>(&self) -> tui::text::Span<'a> {
        Span::styled("⡵", Style::default().fg(Color::LightRed).bg(Color::DarkGray))
    }

    fn collect(&mut self) -> ItemKind {
        self.life -= 1;
        ItemKind::Wood(Wood::new(2))
    }

    fn is_compatible_tool(item: ItemKind) -> bool {
        match item {
            ItemKind::Hand(_) => true,
            ItemKind::Pickaxe(_) => true,
            _ => false
        }
    }

    fn is_destroyed(&self) -> bool {
        self.life == 0
    }
}
