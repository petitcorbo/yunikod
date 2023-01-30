use tui::{widgets::{canvas::Context, ListItem}, text::{Span, Spans}};
use std::{ops::{Index, IndexMut}, mem::discriminant};
use crate::items::ItemKind;
use self::{bullet::Bullet, fire::Fire, swing::Swing};

pub mod player;
pub mod bullet;
pub mod fire;
pub mod swing;

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Recipe {
    Axe,
    Pickaxe,
    Boat,
    Armor,
    Sword,
    Bow,
    Arrow
}

impl Recipe {
    pub fn needs(&self) -> Vec<(ItemKind, u32)> {
        match self {
            Recipe::Pickaxe => Vec::new(),
            Recipe::Axe => Vec::new(),
            Recipe::Arrow => Vec::new(),
            Recipe::Armor => Vec::new(),
            Recipe::Bow => Vec::new(),
            Recipe::Boat => Vec::new(),
            Recipe::Sword => Vec::new()
        }
    }

    pub fn is_craftable(&self, inventory: Inventory) -> bool {
        for (item, amount) in self.needs() {
            if inventory.total_quantity(item) < amount {
                return false;
            }
        }
        true
    }

    pub fn information(&self) -> Spans {
        unimplemented!()
    }

    pub fn recipes() -> Vec<Recipe> {
        vec![
            Recipe::Pickaxe,
            Recipe::Axe,
            Recipe::Armor,
            Recipe::Arrow,
            Recipe::Boat,
            Recipe::Sword,
        ]
    }
}

pub struct Inventory(Vec<ItemKind>);

impl Inventory {
    /// Create new empty inventory
    pub fn new() -> Self {
        Inventory(Vec::new())
    }

    /// add an item to the inventory
    pub fn add(&mut self, mut item_to_add: ItemKind) {
        for item in &mut self.0 {
            if discriminant(item) == discriminant(&mut item_to_add) {
                item.change_quantity(item_to_add.quantity());
                return;
            }
        }
        self.0.push(item_to_add);
    }

    pub fn total_quantity(&self, item_type: ItemKind) -> u32 {
        let mut total: u32 = 0;
        for item in &self.0 {
            if discriminant(item) == discriminant(&item_type) {
                total += item.quantity() as u32;
            }
        }
        total
    }

    pub fn craft(&mut self, recipe: Recipe) -> bool {
        for (item, amount) in recipe.needs() {
            unimplemented!();
        }
        true
    }

    pub fn get(&mut self, index: usize) -> &mut ItemKind {
        &mut self.0[index]
    }

    pub fn to_item_list(&self) -> Vec<ListItem> {
        let mut listitem = Vec::new();
        for item in &self.0 {
            listitem.push(ListItem::new(item.shape()));
        }
        listitem
    }

    pub fn to_extended_item_list(&self) -> Vec<ListItem> {
        let mut listitem = Vec::new();
        for item in &self.0 {
            let spans = Spans::from(vec![
                item.shape(),
                Span::from(item.name())
            ]);
            listitem.push(ListItem::new(spans));
        }
        listitem
    }

    pub fn len(&self) -> usize {
        self.0.len()
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
    Bullet(Bullet),
    Fire(Fire),
    Swing(Swing),
}

impl EntityKind {
    pub fn on_tick(&mut self) {
        match self {
            EntityKind::Bullet(e) => e.on_tick(),
            EntityKind::Fire(e) => e.on_tick(),
            EntityKind::Swing(e) => e.on_tick(),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        match self {
            EntityKind::Bullet(e) => e.draw(ctx),
            EntityKind::Fire(e) => e.draw(ctx),
            EntityKind::Swing(e) => e.draw(ctx),
        };
    }

    pub fn looking_at(&mut self) -> (f64, f64, Direction) {
        match self {
            EntityKind::Bullet(e) => e.looking_at(),
            EntityKind::Fire(e) => e.looking_at(),
            EntityKind::Swing(e) => e.looking_at(),
        }
    }
    
    pub fn looking(&mut self) -> Direction {
        match self {
            EntityKind::Bullet(e) => e.looking(),
            EntityKind::Fire(e) => e.looking(),
            EntityKind::Swing(e) => e.looking(),
        }
    }

    pub fn is_dead(&self) -> bool {
        match self {
            EntityKind::Bullet(e) => e.is_dead(),
            EntityKind::Fire(e) => e.is_dead(),
            EntityKind::Swing(e) => e.is_dead(),
        }
    }

    pub fn collide(&self, x: f64, y: f64) -> bool {
        match self {
            EntityKind::Bullet(e) => e.collide(x, y),
            EntityKind::Fire(e) => e.collide(x, y),
            EntityKind::Swing(e) => e.collide(x, y),
        }
    }

    pub fn is_harmful(&self) -> bool {
        match self {
            EntityKind::Bullet(e) => e.is_harmful(),
            EntityKind::Fire(e) => e.is_harmful(),
            EntityKind::Swing(e) => e.is_harmful(),
        }
    }

    pub fn damage(&self) -> u8 {
        match self {
            EntityKind::Bullet(e) => e.damage(),
            EntityKind::Fire(e) => e.damage(),
            EntityKind::Swing(e) => e.damage(),
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
    fn is_harmful(&self) -> bool;
    fn damage(&self) -> u8;
    fn looking_at(&mut self) -> (f64, f64, Direction) {
        match self.looking() {
            Direction::Up => (self.x(), self.y() + 1.0, Direction::Up),
            Direction::Down => (self.x(), self.y() - 1.0, Direction::Down),
            Direction::Left => (self.x() - 1.0, self.y(), Direction::Left),
            Direction::Right => (self.x() + 1.0, self.y(), Direction::Right),
        }
    }

    fn collide(&self, x: f64, y: f64) -> bool {
        if self.x() <= x && x <= self.x() + 1.0 && self.y() <= y && y <= self.y() + 1.0 {
            true
        } else { false }
    }
}
