use tui::{    
    style::{Color, Style},
    text::{Span, Spans}, widgets::canvas::Context,
};
use crate::{entities::{Direction, EntityKind}, items::ItemKind};

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
    immunity: u8,
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
            max_life: 50,
            immunity: 0,
        }
    }

    pub fn inventory(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    pub fn go(&mut self, direction: Direction, entities: &Vec<EntityKind>) {
        let mut x = self.x;
        let mut y = self.y;
        match direction {
            Direction::Up => y += 1.0,
            Direction::Down => y -= 1.0,
            Direction::Left => x -= 1.0,
            Direction::Right => x += 1.0,
        }
        let mut can_move = true;
        for entity in entities {
            if entity.collide(x, y) {
                can_move = false;
                if entity.is_harmful() {
                    self.hurt(entity.damage());
                }
                break;
            }
        }
        if can_move {
            self.x = x;
            self.y = y;
        }
        self.looking = direction;
    }

    pub fn on_space(&mut self) -> Option<EntityKind> {
        let coords = self.looking_at().clone();
        if let &mut Some(item) = &mut self.inventory.0.get(self.using) {
            item.utilize(coords)
        } else {
            None
        }
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
            Direction::Up => "◓",
            Direction::Down => "◒",
            Direction::Left => "◐",
            Direction::Right => "◑",
        };
        Span::styled(sprite, self.style)
    }

    pub fn is_dead(&self) -> bool {
        self.life == 0
    }

    pub fn looking(&mut self) -> Direction {
        self.looking.clone()
    }

    pub fn on_tick(&mut self) {
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

    pub fn health_bar(&self) -> Spans<'a> {
        let mut bar = "█".repeat((self.life/4) as usize);
        bar.push(match self.life%4 {
            1 => '▎',
            2 => '▌',
            3 => '▊',
            _ => '█',
        });
        bar.push_str(" ".repeat((self.max_life - self.life) as usize).as_str());
        Spans::from(vec![
            Span::raw("["),
            Span::styled(bar, Style::default().fg(Color::Red)),
            Span::raw("]"),
        ])
    }
}
