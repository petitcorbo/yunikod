use crossterm::event::{KeyEvent, KeyModifiers};
use tui::{ 
    style::{Color, Style},
    text::Span,
    widgets::canvas::Context,
};
use crate::{entities::{Direction, EntityKind}, items::ItemKind, game::Game, chunk::Terrain};

use super::Inventory;

pub struct Player {
    pub x: f64,
    pub y: f64,
    looking: Direction,
    moving: bool,
    style: Style,
    inventory: Inventory,
    using: usize,
    life: u8,
    max_life: u8,
    immunity: u8,
}

impl<'a> Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            x,
            y,
            looking: Direction::Up,
            moving: false,
            style: Style::default().fg(Color::Black),
            inventory: Inventory::new(),
            using: 0,
            life: 100,
            max_life: 100,
            immunity: 0,
        }
    }

    pub fn inventory(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    pub fn using(&self) -> usize {
        self.using
    }

    pub fn step(&mut self, game: &mut Game) {
        let mut x = self.x;
        let mut y = self.y;
        match self.looking {
            Direction::Up => y += 1.0,
            Direction::Down => y -= 1.0,
            Direction::Left => x -= 1.0,
            Direction::Right => x += 1.0,
        }
        let mut can_move = true;
        for entity in game.entities() {
            if entity.collide(x, y) {
                can_move = false;
                if entity.is_harmful() {
                    self.hurt(entity.damage());
                }
                break;
            }
        }
        match game.get_tile(x, y) {
            Terrain::Water => can_move = false,
            Terrain::DeepWater => can_move = false,
            _ => {},
        }
        if can_move && game.get_block(x, y).is_none() {
            self.x = x;
            self.y = y;
        }
    }

    pub fn on_arrow(&mut self, key: &KeyEvent, direction: Direction) {
        self.look(direction);
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            self.moving(false);
        } else {
            self.moving(true);
        }
    }

    pub fn on_space(&mut self, game: &mut Game) -> Option<EntityKind> {
        let (x, y, _) = self.looking_at();
        let mut message = String::new();
        if let &mut Some(item) = &mut self.inventory.0.get(self.using) {
            if let Some(block) = game.get_block(x, y) {
                let item = block.collect();
                message = format!("collected {} x{}", item.name(), item.quantity());
                self.inventory.add(item);
                if block.is_destroyed() {
                    game.destroy_block(x, y);
                }
            } else {
                return item.utilize((x, y, self.looking.to_owned()));
            }
        }
        game.set_message(message);
        None
    }

    pub fn look(&mut self, direction: Direction) {
        self.looking = direction;
    }

    pub fn moving(&mut self, moving: bool) {
        self.moving = moving;
    }

    pub fn is_moving(&self) -> bool {
        self.moving
    }

    fn looking_at(&mut self) -> (f64, f64, Direction) {
        match self.looking() {
            Direction::Up => (self.x(), self.y() + 1.0, Direction::Up),
            Direction::Down => (self.x(), self.y() - 1.0, Direction::Down),
            Direction::Left => (self.x() - 1.0, self.y(), Direction::Left),
            Direction::Right => (self.x() + 1.0, self.y(), Direction::Right),
        }
    }

    pub fn pick_up(&mut self, item: ItemKind) {
        self.inventory.add(item);
    }

    pub fn draw<'b>(&'a self, ctx: &mut Context<'b>) {
        ctx.print(self.x, self.y, self.shape())
    }

    pub fn x(&self) -> f64 {
        self.x.floor()
    }

    pub fn y(&self) -> f64 {
        self.y.floor()
    }

    pub fn shape(&self) -> Span<'a> {
        let sprite = match self.looking {
            Direction::Up => "▲",
            Direction::Down => "▼",
            Direction::Left => "◀",
            Direction::Right => "▶",
        };
        Span::styled(sprite, self.style)
    }

    pub fn is_dead(&self) -> bool {
        self.life == 0
    }

    pub fn looking(&mut self) -> Direction {
        self.looking.clone()
    }

    pub fn on_tick(&mut self, game: &mut Game) {
        if self.moving {
            self.step(game)
        }
        if self.immunity > 0 {
            self.immunity -= 1;
        }
    }

    pub fn heal(&mut self, amount: u8) {
        self.life += amount;
        if self.life > self.max_life {
            self.life = self.max_life;
        }
    }

    pub fn hurt(&mut self, amount: u8) {
        if self.life >= amount && self.immunity == 0 {
            self.life -= amount;
            self.immunity = 5;
        }
    }

    pub fn life_ratio(&self) -> f64 {
        (self.life as f64) / (self.max_life as f64)
    }
}
