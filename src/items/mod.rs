pub mod gun;

use gun::Gun;
use crate::entities::{EntityKind, Direction};

pub enum ItemKind {
    Gun(Gun),
}

impl ItemKind {
    pub fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind> {
        match self {
            ItemKind::Gun(i) => i.utilize(coords),
        }
    }
}

pub trait Item {
    fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind>;
}
