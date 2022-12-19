pub mod gun;
pub mod flamethrower;
pub mod wood;
pub mod stick;
pub mod stone;
pub mod grass;
pub mod coal;
pub mod gold;
pub mod iron;
pub mod pickaxe;
pub mod axe;
pub mod hand;

use crate::entities::{EntityKind, Direction};
use gun::Gun;
use flamethrower::FlameThrower;
use tui::text::Span;
use wood::Wood;

use self::{gold::Gold, iron::Iron, coal::Coal, grass::Grass, stick::Stick, stone::Stone, pickaxe::Pickaxe, axe::Axe, hand::Hand};

pub enum ItemKind {
    Gun(Gun),
    FT(FlameThrower),
    Wood(Wood),
    Gold(Gold),
    Iron(Iron),
    Stone(Stone),
    Coal(Coal),
    Grass(Grass),
    Stick(Stick),
    Pickaxe(Pickaxe),
    Axe(Axe),
    Hand(Hand)
}

impl ItemKind {
    pub fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind> {
        match self {
            ItemKind::Gun(i) => i.utilize(coords),
            ItemKind::FT(i) => i.utilize(coords),
            ItemKind::Stick(i) => i.utilize(coords),
            ItemKind::Stone(i) => i.utilize(coords),
            ItemKind::Gold(i) => i.utilize(coords),
            ItemKind::Grass(i) => i.utilize(coords),
            ItemKind::Wood(i) => i.utilize(coords),
            ItemKind::Iron(i) => i.utilize(coords),
            ItemKind::Coal(i) => i.utilize(coords),
            ItemKind::Pickaxe(i) => i.utilize(coords),
            ItemKind::Axe(i) => i.utilize(coords),
            ItemKind::Hand(i) => i.utilize(coords),
        }
    }

    pub fn shape<'a>(&self) -> Span<'a> {
        match self {
            ItemKind::Gun(_) => Gun::shape(),
            ItemKind::FT(_) => FlameThrower::shape(),
            ItemKind::Stick(_) => Stick::shape(),
            ItemKind::Stone(_) => Stone::shape(),
            ItemKind::Gold(_) => Gold::shape(),
            ItemKind::Grass(_) => Grass::shape(),
            ItemKind::Wood(_) => Wood::shape(),
            ItemKind::Iron(_) => Iron::shape(),
            ItemKind::Coal(_) => Coal::shape(),
            ItemKind::Pickaxe(_) => Pickaxe::shape(),
            ItemKind::Axe(_) => Axe::shape(),
            ItemKind::Hand(_) => Hand::shape(),
        }
    }
}

pub trait Item {
    fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind>;
    fn shape<'a>() -> Span<'a>;
}
