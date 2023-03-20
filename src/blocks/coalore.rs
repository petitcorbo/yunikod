use tui::{text::Span, style::{Style, Color}};
use crate::items::{ItemKind, coal::Coal};
use super::{Block, BlockKind};

pub struct CoalOre {
    life: u32,
}

impl Block for CoalOre {
    fn generate() -> BlockKind {
        BlockKind::CoalOre(
            Self {
                life: 15
            }
        )
    }

    fn shape<'a>(&self) -> tui::text::Span<'a> {
        Span::styled("⣳", Style::default().fg(Color::Black).bg(Color::DarkGray))
    }

    fn collect(&mut self) -> ItemKind {
        self.life -= 1;
        ItemKind::Coal(Coal::new(2))
    }

    fn is_compatible_tool(item: &ItemKind) -> bool {
        match item {
            ItemKind::Pickaxe(_) => true,
            _ => false
        }
    }

    fn is_destroyed(&self) -> bool {
        self.life == 0
    }
}
