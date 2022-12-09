use tui::{    
    style::{Color, Style},
    text::Span, widgets::canvas::Context,
};
use crate::{entities::{Direction, Entity, EntityKind}, items::ItemKind};

use super::Inventory;

pub struct Player {
    pub x: f64,
    pub y: f64,
    looking: Direction,
    style: Style,
    inventory: Inventory,
    using: usize,
    life: u8,
    max_life: u8,
}

impl<'a> Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            x,
            y,
            looking: Direction::Up,
            style: Style::default().fg(Color::Cyan),
            inventory: Inventory::new(),
            using: 0,
            life: 50,
            max_life: 50
        }
    }

    pub fn inventory(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    pub fn on_up(&mut self) {
        self.y += 1.0;
        self.looking = Direction::Up;
    }

    pub fn on_down(&mut self) {
        self.y -= 1.0;
        self.looking = Direction::Down;
    }

    pub fn on_left(&mut self) {
        self.x -= 1.0;
        self.looking = Direction::Left;
    }

    pub fn on_right(&mut self) {
        self.x += 1.0;
        self.looking = Direction::Right;
    }

    pub fn on_space(&mut self) -> Option<EntityKind> {
        let coords = self.looking_at().clone();
        if let &mut Some(item) = &mut self.inventory.0.get(self.using) {
            item.utilize(coords)
        } else {
            None
        }
    }

    pub fn pick_up(&mut self, item: ItemKind) {
        self.inventory.add(item);
    }
}

impl<'a> Entity<'a> for Player {
    fn draw<'b>(&'a self, ctx: &mut Context<'b>) {
        ctx.print(self.x, self.y, self.shape())
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn shape(&self) -> Span<'a> {
        let sprite = match self.looking {
            Direction::Up => "◓",
            Direction::Down => "◒",
            Direction::Left => "◐",
            Direction::Right => "◑",
        };
        Span::styled(sprite, self.style)
    }

    fn is_dead(&self) -> bool {
        self.life == 0
    }

    fn looking_at(&mut self) -> (f64, f64, Direction) {
        match self.looking {
            Direction::Up => (self.x, self.y + 1.0, Direction::Up),
            Direction::Down => (self.x, self.y - 1.0, Direction::Down),
            Direction::Left => (self.x - 1.0, self.y, Direction::Left),
            Direction::Right => (self.x + 1.0, self.y, Direction::Right),
        }
    }

    fn looking(&mut self) -> Direction {
        self.looking.clone()
    }

    fn on_tick(&mut self) {
        ()
    }

    fn heal(&mut self, amount: u8) {
        self.life += amount;
        if self.life > self.max_life {
            self.life = self.max_life;
        }
    }

    fn hurt(&mut self, amount: u8) {
        self.life -= amount;
    }
}
