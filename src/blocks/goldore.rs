use tui::{text::Span, style::{Style, Color}};
use crate::items::{ItemKind, gold::Gold};
use super::{Block, BlockKind};

pub struct GoldOre {
    life: u32,
}

impl Block for GoldOre {
    fn generate() -> BlockKind {
        BlockKind::GoldOre(
            Self {
                life: 15
            }
        )
    }

    fn shape<'a>(&self) -> tui::text::Span<'a> {
        Span::styled("⡝", Style::default().fg(Color::Green).bg(Color::Yellow))
    }

    fn collect(&mut self) -> ItemKind {
        self.life -= 1;
        ItemKind::Gold(Gold::new(2))
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
