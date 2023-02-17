use tui::{widgets::canvas::Context, text::Span};
use crate::game::Game;
use self::{onyxstone::OnyxStone, fire::Fire, swing::Swing, player::Player, snake::Snake};

pub mod player;
pub mod onyxstone;
pub mod fire;
pub mod swing;
pub mod snake;

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum EntityKind {
    OnyxStone(OnyxStone),
    Fire(Fire),
    Swing(Swing),
    Snake(Snake),
}

pub enum Action {
    Move(f64, f64),
    Attack(usize, u8),
    Spawn(Vec<EntityKind>),
    Nothing
}

impl EntityKind {
    pub fn on_tick(&mut self) {
        match self {
            EntityKind::OnyxStone(e) => e.on_tick(),
            EntityKind::Fire(e) => e.on_tick(),
            EntityKind::Swing(e) => e.on_tick(),
            EntityKind::Snake(e) => e.on_tick(),
        }
    }

    pub fn go(&mut self, x: f64, y: f64) {
        match self {
            EntityKind::OnyxStone(e) => e.go(x, y),
            EntityKind::Fire(e) => e.go(x, y),
            EntityKind::Swing(e) => e.go(x, y),
            EntityKind::Snake(e) => e.go(x, y),
        }
    }

    pub fn hurt(&mut self, amount: u8) {
        match self {
            EntityKind::OnyxStone(e) => e.hurt(amount),
            EntityKind::Fire(e) => e.hurt(amount),
            EntityKind::Swing(e) => e.hurt(amount),
            EntityKind::Snake(e) => e.hurt(amount),
        }
    }

    pub fn on_action(&self, player: &mut Player, game: &Game) -> Action {
        match self {
            EntityKind::OnyxStone(e) => e.on_action(player, game),
            EntityKind::Fire(e) => e.on_action(player, game),
            EntityKind::Swing(e) => e.on_action(player, game),
            EntityKind::Snake(e) => e.on_action(player, game),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        match self {
            EntityKind::OnyxStone(e) => e.draw(ctx),
            EntityKind::Fire(e) => e.draw(ctx),
            EntityKind::Swing(e) => e.draw(ctx),
            EntityKind::Snake(e) => e.draw(ctx),
        };
    }

    pub fn looking_at(&mut self) -> (f64, f64, Direction) {
        match self {
            EntityKind::OnyxStone(e) => e.looking_at(),
            EntityKind::Fire(e) => e.looking_at(),
            EntityKind::Swing(e) => e.looking_at(),
            EntityKind::Snake(e) => e.looking_at(),
        }
    }
    
    pub fn looking(&mut self) -> Direction {
        match self {
            EntityKind::OnyxStone(e) => e.looking(),
            EntityKind::Fire(e) => e.looking(),
            EntityKind::Swing(e) => e.looking(),
            EntityKind::Snake(e) => e.looking(),
        }
    }

    pub fn is_dead(&self) -> bool {
        match self {
            EntityKind::OnyxStone(e) => e.is_dead(),
            EntityKind::Fire(e) => e.is_dead(),
            EntityKind::Swing(e) => e.is_dead(),
            EntityKind::Snake(e) => e.is_dead(),
        }
    }

    pub fn collide(&self, x: f64, y: f64) -> bool {
        match self {
            EntityKind::OnyxStone(e) => e.collide(x, y),
            EntityKind::Fire(e) => e.collide(x, y),
            EntityKind::Swing(e) => e.collide(x, y),
            EntityKind::Snake(e) => e.collide(x, y),
        }
    }

    pub fn is_harmful(&self) -> bool {
        match self {
            EntityKind::OnyxStone(e) => e.is_harmful(),
            EntityKind::Fire(e) => e.is_harmful(),
            EntityKind::Swing(e) => e.is_harmful(),
            EntityKind::Snake(e) => e.is_harmful(),
        }
    }

    pub fn damage(&self) -> u8 {
        match self {
            EntityKind::OnyxStone(e) => e.damage(),
            EntityKind::Fire(e) => e.damage(),
            EntityKind::Swing(e) => e.damage(),
            EntityKind::Snake(e) => e.damage(),
        }
    }
}

pub trait Entity<'a> {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn go(&mut self, x: f64, y: f64);
    fn shape(&self) -> Span<'a>;
    fn draw<'b>(&'a self, ctx: &mut Context<'b>);
    fn on_tick(&mut self);
    fn on_action(&self, player: &mut Player, game: &Game) -> Action;
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
