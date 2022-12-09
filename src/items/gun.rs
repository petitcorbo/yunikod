use crate::{
    items::Item,
    entities::{
        EntityKind,
        bullet::Bullet, Direction,
    }
};

pub struct Gun;

impl Item for Gun {
    fn utilize(&self, coords: (f64, f64, Direction)) -> Option<EntityKind> {
        let (x, y, direction) = coords;
        Some(EntityKind::Bullet(Bullet::new(x, y, direction)))
    }
}
