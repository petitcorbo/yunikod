pub mod tree;
pub mod grasstuft;
pub mod stones;
pub mod ironore;
pub mod goldore;
pub mod coalore;
pub mod rock;
pub mod sticks;

use tui::text::Span;
use crate::items::ItemKind;

use self::{tree::Tree, rock::Rock, grasstuft::GrassTuft, goldore::GoldOre, ironore::IronOre, coalore::CoalOre, stones::Stones, sticks::Sticks};

pub enum BlockKind {
    Tree(Tree),
    Grass(GrassTuft),
    GoldOre(GoldOre),
    IronOre(IronOre),
    CoalOre(CoalOre),
    Stones(Stones),
    Rock(Rock),
    Sticks(Sticks)
}

impl BlockKind {
    pub fn shape<'a>(&self) -> Span<'a> {
        match self {
            BlockKind::Tree(b) => b.shape(),
            BlockKind::Grass(b) => b.shape(),
            BlockKind::GoldOre(b) => b.shape(),
            BlockKind::IronOre(b) => b.shape(),
            BlockKind::CoalOre(b) => b.shape(),
            BlockKind::Stones(b) => b.shape(),
            BlockKind::Rock(b) => b.shape(),
            BlockKind::Sticks(b) => b.shape()
        }
    }
    
    pub fn collect(&mut self) -> ItemKind {
        match self {
            BlockKind::Tree(b) => b.collect(),
            BlockKind::Grass(b) => b.collect(),
            BlockKind::GoldOre(b) => b.collect(),
            BlockKind::IronOre(b) => b.collect(),
            BlockKind::CoalOre(b) => b.collect(),
            BlockKind::Stones(b) => b.collect(),
            BlockKind::Rock(b) => b.collect(),
            BlockKind::Sticks(b) => b.collect()
        }
    }

    pub fn is_destroyed(&self) -> bool {
        match self {
            BlockKind::Tree(b) => b.is_destroyed(),
            BlockKind::Grass(b) => b.is_destroyed(),
            BlockKind::GoldOre(b) => b.is_destroyed(),
            BlockKind::IronOre(b) => b.is_destroyed(),
            BlockKind::CoalOre(b) => b.is_destroyed(),
            BlockKind::Stones(b) => b.is_destroyed(),
            BlockKind::Rock(b) => b.is_destroyed(),
            BlockKind::Sticks(b) => b.is_destroyed()
        }
    }

    pub fn is_compatible_tool(&self, item: ItemKind) -> bool {
        match &self {
            BlockKind::Tree(_) => Tree::is_compatible_tool(item),
            BlockKind::Grass(_) => GrassTuft::is_compatible_tool(item),
            BlockKind::GoldOre(_) => GoldOre::is_compatible_tool(item),
            BlockKind::IronOre(_) => IronOre::is_compatible_tool(item),
            BlockKind::CoalOre(_) => CoalOre::is_compatible_tool(item),
            BlockKind::Stones(_) => Stones::is_compatible_tool(item),
            BlockKind::Rock(_) => Rock::is_compatible_tool(item),
            BlockKind::Sticks(_) => Rock::is_compatible_tool(item)
        }
    }
}

pub trait Block {
    fn generate() -> BlockKind;
    fn collect(&mut self) -> ItemKind;
    fn shape<'a>(&self) -> Span<'a>;
    fn is_compatible_tool(item: ItemKind) -> bool;
    fn is_destroyed(&self) -> bool;
}
