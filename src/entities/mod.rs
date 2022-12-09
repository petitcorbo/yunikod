use tui::{widgets::canvas::Context, text::Span};
use std::ops::{Index, IndexMut};
use crate::items::ItemKind;
use self::{bullet::Bullet, player::Player};

pub mod player;
pub mod bullet;

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Inventory(Vec<ItemKind>);

impl Inventory {
    /// Create new empty inventory
    pub fn new() -> Self {
        Inventory(Vec::new())
    }
    
    /// add an item to the inventory
    pub fn add(&mut self, item: ItemKind) {
        self.0.push(item);
    }

    pub fn get(&mut self, index: usize) -> &mut ItemKind {
        &mut self.0[index]
    }
}

impl Index<usize> for Inventory {
    type Output = ItemKind;

    fn index(&self, i: usize) -> &ItemKind {
        &self.0[i]
    }
}

impl IndexMut<usize> for Inventory {
    fn index_mut(&mut self, i: usize) -> &mut ItemKind {
        &mut self.0[i]
    }
}

pub enum EntityKind {
    Player(Player),
    Bullet(Bullet),
}

impl EntityKind {
    pub fn on_tick(&mut self) {
        match self {
            EntityKind::Player(e) => e.on_tick(),
            EntityKind::Bullet(e) => e.on_tick(),
        };
    }

    pub fn draw(&self, ctx: &mut Context) {
        match self {
            EntityKind::Player(e) => e.draw(ctx),
            EntityKind::Bullet(e) => e.draw(ctx),
        };
    }

    pub fn looking_at(&mut self) -> (f64, f64, Direction) {
        match self {
            EntityKind::Player(e) => e.looking_at(),
            EntityKind::Bullet(e) => e.looking_at(),
        }
    }
    
    pub fn looking(&mut self) -> Direction {
        match self {
            EntityKind::Player(e) => e.looking(),
            EntityKind::Bullet(e) => e.looking(),
        }
    }

    pub fn is_dead(&self) -> bool {
        match self {
            EntityKind::Player(e) => e.is_dead(),
            EntityKind::Bullet(e) => e.is_dead(),
        }
    }
}

pub trait Entity<'a> {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn shape(&self) -> Span<'a>;
    fn draw<'b>(&'a self, ctx: &mut Context<'b>);
    fn on_tick(&mut self);
    fn is_dead(&self) -> bool;
    fn hurt(&mut self, amount: u8);
    fn heal(&mut self, amount: u8);
    fn looking(&mut self) -> Direction;
    fn looking_at(&mut self) -> (f64, f64, Direction) {
        match self.looking() {
            Direction::Up => (self.x(), self.y() + 1.0, Direction::Up),
            Direction::Down => (self.x(), self.y() - 1.0, Direction::Down),
            Direction::Left => (self.x() - 1.0, self.y(), Direction::Left),
            Direction::Right => (self.x() + 1.0, self.y(), Direction::Right),
        }
    }
}
