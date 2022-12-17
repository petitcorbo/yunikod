use tui::{text::Span, style::{Color, Style}};
use crate::items::{ItemKind, wood::Wood, stone::Stone};
use super::Block;

pub struct Rock {
    life: u8,
}

impl Block for Rock {
    fn shape<'a>(&self) -> tui::text::Span<'a> {
        let style = Style::default()
            .fg(Color::DarkGray)
            .bg(Color::Gray);
        Span::styled("⣿", style)
    }

    fn collect(&mut self) -> ItemKind {
        self.life -= 1;
        ItemKind::Stone(Stone::new(2))
    }

    fn is_compatible_tool(tool: ItemKind) -> bool {
        if let ItemKind::Pickaxe(_) = tool {
            true
        } else {
            false
        }
    }
}
