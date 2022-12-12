use crate::{
    items::Item,
    entities::{
        EntityKind,
        fire::Fire, Direction,
    }
};

pub struct FlameThrower;

impl Item for FlameThrower {
    fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind> {
        let (x, y, direction) = coords;
        Some(EntityKind::Fire(Fire::new(x, y, direction)))
    }
}
