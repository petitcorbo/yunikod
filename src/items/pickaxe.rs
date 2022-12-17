use super::Item;

pub struct Pickaxe {
    quantity: u32,
}

impl Pickaxe {
    pub fn new(quantity: u32) -> Self {
        Self {
            quantity
        }
    }
}

impl Item for Pickaxe {
    fn utilize(&self, coords: (f64, f64, crate::entities::Direction)) -> Option<crate::entities::EntityKind> {
        None
    }
}
