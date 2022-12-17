pub mod tree;
pub mod grass;
pub mod stone;
pub mod iron;
pub mod gold;
pub mod coal;
pub mod rock;

use tui::text::Span;
use crate::items::{ItemKind, grass::Grass, gold::Gold, iron::Iron, coal::Coal, stone::Stone};

use self::tree::Tree;

pub enum BlockKind {
    Tree(Tree),
    Grass(Grass),
    Gold(Gold),
    Iron(Iron),
    Coal(Coal),
    Stone(Stone)
}

impl BlockKind {
    pub fn shape<'a>(&self) -> Span<'a> {
        match &self {
            BlockKind::Tree(b) => b.shape(),
            BlockKind::Grass(b) => b.shape(),
            BlockKind::Gold(b) => b.shape(),
            BlockKind::Iron(b) => b.shape(),
            BlockKind::Coal(b) => b.shape(),
            BlockKind::Stone(b) => b.shape()
        }
    }
    
    pub fn collect(&self) -> ItemKind {
        match &self {
            BlockKind::Tree(b) => b.collect(),
            BlockKind::Grass(b) => b.collect(),
            BlockKind::Gold(b) => b.collect(),
            BlockKind::Iron(b) => b.collect(),
            BlockKind::Coal(b) => b.collect(),
            BlockKind::Stone(b) => b.collect()
        }
    }

    pub fn is_compatible_tool(&self, item: ItemKind) -> bool {
        match &self {
            BlockKind::Tree(_) => Tree::is_compatible_tool(item),
            BlockKind::Grass(_) => Grass::is_compatible_tool(),
            BlockKind::Gold(_) => Gold::is_compatible_tool(),
            BlockKind::Iron(_) => Iron::is_compatible_tool(),
            BlockKind::Coal(_) => Coal::is_compatible_tool(),
            BlockKind::Stone(_) => Stone::is_compatible_tool(item)
        }
    }
}

pub trait Block {
    fn collect(&mut self) -> ItemKind;
    fn shape<'a>(&self) -> Span<'a>;
    fn is_compatible_tool(item: ItemKind) -> bool;
}
