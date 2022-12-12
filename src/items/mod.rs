pub mod gun;
pub mod flamethrower;

use gun::Gun;
use flamethrower::FlameThrower;
use crate::entities::{EntityKind, Direction};

pub enum ItemKind {
    Gun(Gun),
    FT(FlameThrower),
}

impl ItemKind {
    pub fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind> {
        match self {
            ItemKind::Gun(i) => i.utilize(coords),
            ItemKind::FT(i) => i.utilize(coords),
        }
    }
}

pub trait Item {
    fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind>;
}
