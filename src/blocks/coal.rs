use tui::{text::Span, style::Style};
use crate::items::{ItemKind, wood::Wood};
use super::Block;

struct Coal {
    life: u32,
}

impl Block for Coal {
    fn shape<'a>(&self) -> tui::text::Span<'a> {
        Span::styled("$", Style::default().fg(tui::style::Color::Green))
    }

    fn collect(&mut self) -> ItemKind {
        self.life -= 1;
        ItemKind::Wood(Wood::new(2))
    }

    fn is_compatible_tool(item: ItemKind) -> bool {
        match item {
            ItemKind::Pickaxe(_) => true,
            _ => false
        }
    }
}
