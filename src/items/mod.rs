pub mod onyxheart;
pub mod dragonsoul;
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
pub mod bow;

use crate::entities::{EntityKind, Direction};
use dragonsoul::DragonSoul;
use onyxheart::OnyxHeart;
use tui::text::Span;
use wood::Wood;

use self::{gold::Gold, iron::Iron, coal::Coal, grass::Grass, stick::Stick, stone::Stone, pickaxe::Pickaxe, axe::Axe, hand::Hand, bow::Bow};

pub enum ItemKind {
    OH(OnyxHeart),
    DS(DragonSoul),
    Wood(Wood),
    Gold(Gold),
    Iron(Iron),
    Stone(Stone),
    Coal(Coal),
    Grass(Grass),
    Stick(Stick),
    Pickaxe(Pickaxe),
    Axe(Axe),
    Hand(Hand),
    Bow(Bow)
}

impl ItemKind {
    pub fn utilize(&self, coords: (i64, i64, Direction)) -> Option<EntityKind> {
        match self {
            ItemKind::OH(i) => i.utilize(coords),
            ItemKind::DS(i) => i.utilize(coords),
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
            ItemKind::Bow(i) => i.utilize(coords),
        }
    }

    pub fn damage(&self) -> u8 {
        match self {
            ItemKind::OH(i) => i.damage(),
            ItemKind::DS(i) => i.damage(),
            ItemKind::Stick(i) => i.damage(),
            ItemKind::Stone(i) => i.damage(),
            ItemKind::Gold(i) => i.damage(),
            ItemKind::Grass(i) => i.damage(),
            ItemKind::Wood(i) => i.damage(),
            ItemKind::Iron(i) => i.damage(),
            ItemKind::Coal(i) => i.damage(),
            ItemKind::Pickaxe(i) => i.damage(),
            ItemKind::Axe(i) => i.damage(),
            ItemKind::Hand(i) => i.damage(),
            ItemKind::Bow(i) => i.damage(),
        }
    }

    pub fn shape<'a>(&self) -> Span<'a> {
        match self {
            ItemKind::OH(_) => OnyxHeart::shape(),
            ItemKind::DS(_) => DragonSoul::shape(),
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
            ItemKind::Bow(_) => Bow::shape(),
        }
    }

    pub fn name<'a>(&self) -> &str {
        match self {
            ItemKind::OH(_) => OnyxHeart::name(),
            ItemKind::DS(_) => DragonSoul::name(),
            ItemKind::Stick(_) => Stick::name(),
            ItemKind::Stone(_) => Stone::name(),
            ItemKind::Gold(_) => Gold::name(),
            ItemKind::Grass(_) => Grass::name(),
            ItemKind::Wood(_) => Wood::name(),
            ItemKind::Iron(_) => Iron::name(),
            ItemKind::Coal(_) => Coal::name(),
            ItemKind::Pickaxe(_) => Pickaxe::name(),
            ItemKind::Axe(_) => Axe::name(),
            ItemKind::Hand(_) => Hand::name(),
            ItemKind::Bow(_) => Bow::name(),
        }
    }

    pub fn quantity(&self) -> i8 {
        match self {
            ItemKind::OH(i) => i.quantity(),
            ItemKind::DS(i) => i.quantity(),
            ItemKind::Stick(i) => i.quantity(),
            ItemKind::Stone(i) => i.quantity(),
            ItemKind::Gold(i) => i.quantity(),
            ItemKind::Grass(i) => i.quantity(),
            ItemKind::Wood(i) => i.quantity(),
            ItemKind::Iron(i) => i.quantity(),
            ItemKind::Coal(i) => i.quantity(),
            ItemKind::Pickaxe(i) => i.quantity(),
            ItemKind::Axe(i) => i.quantity(),
            ItemKind::Hand(i) => i.quantity(),
            ItemKind::Bow(i) => i.quantity(),
        }
    }
    pub fn change_quantity(&mut self, amount: i8) -> i8 {
        match self {
            ItemKind::OH(i) => i.change_quantity(amount),
            ItemKind::DS(i) => i.change_quantity(amount),
            ItemKind::Stick(i) => i.change_quantity(amount),
            ItemKind::Stone(i) => i.change_quantity(amount),
            ItemKind::Gold(i) => i.change_quantity(amount),
            ItemKind::Grass(i) => i.change_quantity(amount),
            ItemKind::Wood(i) => i.change_quantity(amount),
            ItemKind::Iron(i) => i.change_quantity(amount),
            ItemKind::Coal(i) => i.change_quantity(amount),
            ItemKind::Pickaxe(i) => i.change_quantity(amount),
            ItemKind::Axe(i) => i.change_quantity(amount),
            ItemKind::Hand(i) => i.change_quantity(amount),
            ItemKind::Bow(i) => i.change_quantity(amount),
        }
    }
}

pub trait Item {
    fn utilize(&self, coords: (i64, i64, Direction)) -> Option<EntityKind>;
    fn shape<'a>() -> Span<'a>;
    fn name<'a>() -> &'a str;
    fn quantity(&self) -> i8;
    fn max_quantity(&self) -> i8;
    fn change_quantity(&mut self, amount: i8) -> i8;
    fn damage(&self) -> u8;
}
