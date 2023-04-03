use tui::{text::Span, style::Style};
use crate::items::{ItemKind, grass::Grass};
use super::{Block, BlockKind};

pub struct GrassTuft {
    life: u32,
}

impl Block for GrassTuft {
    fn generate() -> BlockKind {
        BlockKind::Grass(
            Self {
                life: 1
            }
        )
    }

    fn shape<'a>(&self) -> tui::text::Span<'a> {
        Span::styled(";", Style::default().fg(tui::style::Color::Green))
    }

    fn collect(&mut self) -> ItemKind {
        self.life = 0;
        ItemKind::Grass(Grass::new(2))
    }

    fn is_compatible_tool(_item: &ItemKind) -> bool {
        true
    }

    fn is_destroyed(&self) -> bool {
        self.life == 0
    }
}
