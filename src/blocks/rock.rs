use tui::{text::Span, style::{Color, Style}};
use crate::items::{ItemKind, stone::Stone};
use super::{Block, BlockKind};

pub struct Rock {
    life: u8,
}

impl Block for Rock {
    fn generate() -> BlockKind {
        BlockKind::Rock(
            Self {
                life: 15
            }
        )
    }

    fn shape<'a>(&self) -> tui::text::Span<'a> {
        let style = Style::default()
            .fg(Color::Rgb(84, 106, 78))
            .bg(Color::DarkGray);
        Span::styled("⣿", style)
    }

    fn collect(&mut self) -> ItemKind {
        self.life -= 1;
        ItemKind::Stone(Stone::new(2))
    }

    fn is_compatible_tool(tool: &ItemKind) -> bool {
        if let ItemKind::Pickaxe(_) = tool {
            true
        } else {
            false
        }
    }

    fn is_destroyed(&self) -> bool {
        self.life == 0
    }
}
