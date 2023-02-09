use tui::{text::Span, style::{Style, Color}};
use crate::items::{ItemKind, stick::Stick};
use super::{Block, BlockKind};

pub struct Sticks {
    life: u8,
}

impl Block for Sticks {
    fn generate() -> BlockKind {
        BlockKind::Sticks(
            Self {
                life: 1
            }
        )
    }

    fn shape<'a>(&self) -> tui::text::Span<'a> {
        Span::styled("ɻ", Style::default().fg(Color::Rgb(145, 77, 5)))
    }

    fn collect(&mut self) -> ItemKind {
        self.life -= 1;
        ItemKind::Stick(Stick::new(1))
    }

    fn is_compatible_tool(item: ItemKind) -> bool {
        match item {
            ItemKind::Axe(_) => true,
            _ => false
        }
    }

    fn is_destroyed(&self) -> bool {
        self.life == 0
    }
}
