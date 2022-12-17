pub mod gun;
pub mod flamethrower;
pub mod wood;
pub mod stick;
pub mod stone;
pub mod grass;
pub mod coal;
pub mod gold;
pub mod iron;

use crate::entities::{EntityKind, Direction};
use gun::Gun;
use flamethrower::FlameThrower;
use wood::Wood;

use self::{gold::Gold, iron::Iron, coal::Coal, grass::Grass, stick::Stick, stone::Stone};

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
        }
    }
}

pub trait Item {
    fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind>;
}
