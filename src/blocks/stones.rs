use rand::{thread_rng, Rng};
use tui::{text::Span, style::{Style, Color}};
use crate::items::{ItemKind, stone::Stone};
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
                life: thread_rng().gen_range(1..=4)
            }
        )
    }

    fn shape<'a>(&self) -> tui::text::Span<'a> {
        let glyph = match self.life {
            1 => "⠁",
            2 => "⠡",
            3 => "⠣",
            _ => "⠫"
        };
        Span::styled(glyph, Style::default().fg(Color::DarkGray))
    }

    fn collect(&mut self) -> ItemKind {
        if self.life > 0 {
            self.life -= 1;
        }
        ItemKind::Stone(Stone::new(1))
    }

    fn is_compatible_tool(item: &ItemKind) -> bool {
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
