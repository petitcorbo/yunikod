use tui::{widgets::canvas::Context, text::Span};
use crate::game::Game;
use self::{onyxstone::OnyxStone, fire::Fire, swing::Swing, player::Player, snake::Snake, ovis::Ovis, crawler::Crawler, golem::Golem, scorpy::Scorpy};

pub mod player;
pub mod onyxstone;
pub mod fire;
pub mod swing;
pub mod snake;
pub mod ovis;
pub mod scorpy;
pub mod golem;
pub mod crawler;

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
    Ovis(Ovis),
    Scorpy(Scorpy),
    Golem(Golem),
    Crawler(Crawler),
}

pub enum Action {
    Move(i64, i64),
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
            EntityKind::Ovis(e) => e.on_tick(),
            EntityKind::Scorpy(e) => e.on_tick(),
            EntityKind::Golem(e) => e.on_tick(),
            EntityKind::Crawler(e) => e.on_tick(),
        }
    }

    pub fn go(&mut self, x: i64, y: i64) {
        match self {
            EntityKind::OnyxStone(e) => e.go(x, y),
            EntityKind::Fire(e) => e.go(x, y),
            EntityKind::Swing(e) => e.go(x, y),
            EntityKind::Snake(e) => e.go(x, y),
            EntityKind::Ovis(e) => e.go(x, y),
            EntityKind::Scorpy(e) => e.go(x, y),
            EntityKind::Golem(e) => e.go(x, y),
            EntityKind::Crawler(e) => e.go(x, y),
        }
    }

    pub fn hurt(&mut self, amount: u8) {
        match self {
            EntityKind::OnyxStone(e) => e.hurt(amount),
            EntityKind::Fire(e) => e.hurt(amount),
            EntityKind::Swing(e) => e.hurt(amount),
            EntityKind::Snake(e) => e.hurt(amount),
            EntityKind::Ovis(e) => e.hurt(amount),
            EntityKind::Scorpy(e) => e.hurt(amount),
            EntityKind::Golem(e) => e.hurt(amount),
            EntityKind::Crawler(e) => e.hurt(amount),
        }
    }

    pub fn on_action(&self, player: &mut Player, game: &Game) -> Action {
        match self {
            EntityKind::OnyxStone(e) => e.on_action(player, game),
            EntityKind::Fire(e) => e.on_action(player, game),
            EntityKind::Swing(e) => e.on_action(player, game),
            EntityKind::Snake(e) => e.on_action(player, game),
            EntityKind::Ovis(e) => e.on_action(player, game),
            EntityKind::Scorpy(e) => e.on_action(player, game),
            EntityKind::Golem(e) => e.on_action(player, game),
            EntityKind::Crawler(e) => e.on_action(player, game),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        match self {
            EntityKind::OnyxStone(e) => ctx.print(e.x() as f64, e.y() as f64, e.shape()),
            EntityKind::Fire(e) => ctx.print(e.x() as f64, e.y() as f64, e.shape()),
            EntityKind::Swing(e) => ctx.print(e.x() as f64, e.y() as f64, e.shape()),
            EntityKind::Snake(e) => ctx.print(e.x() as f64, e.y() as f64, e.shape()),
            EntityKind::Ovis(e) => ctx.print(e.x() as f64, e.y() as f64, e.shape()),
            EntityKind::Scorpy(e) => ctx.print(e.x() as f64, e.y() as f64, e.shape()),
            EntityKind::Golem(e) => ctx.print(e.x() as f64, e.y() as f64, e.shape()),
            EntityKind::Crawler(e) => ctx.print(e.x() as f64, e.y() as f64, e.shape()),
        };
    }

    pub fn looking_at(&mut self) -> (i64, i64, Direction) {
        match self {
            EntityKind::OnyxStone(e) => e.looking_at(),
            EntityKind::Fire(e) => e.looking_at(),
            EntityKind::Swing(e) => e.looking_at(),
            EntityKind::Snake(e) => e.looking_at(),
            EntityKind::Ovis(e) => e.looking_at(),
            EntityKind::Scorpy(e) => e.looking_at(),
            EntityKind::Golem(e) => e.looking_at(),
            EntityKind::Crawler(e) => e.looking_at(),
        }
    }
    
    pub fn looking(&mut self) -> Direction {
        match self {
            EntityKind::OnyxStone(e) => e.looking(),
            EntityKind::Fire(e) => e.looking(),
            EntityKind::Swing(e) => e.looking(),
            EntityKind::Snake(e) => e.looking(),
            EntityKind::Ovis(e) => e.looking(),
            EntityKind::Scorpy(e) => e.looking(),
            EntityKind::Golem(e) => e.looking(),
            EntityKind::Crawler(e) => e.looking(),
        }
    }

    pub fn is_dead(&self) -> bool {
        match self {
            EntityKind::OnyxStone(e) => e.is_dead(),
            EntityKind::Fire(e) => e.is_dead(),
            EntityKind::Swing(e) => e.is_dead(),
            EntityKind::Snake(e) => e.is_dead(),
            EntityKind::Ovis(e) => e.is_dead(),
            EntityKind::Scorpy(e) => e.is_dead(),
            EntityKind::Golem(e) => e.is_dead(),
            EntityKind::Crawler(e) => e.is_dead(),
        }
    }

    pub fn collide(&self, x: i64, y: i64) -> bool {
        match self {
            EntityKind::OnyxStone(e) => (x, y) == (e.x(), e.y()),
            EntityKind::Fire(e) => (x, y) == (e.x(), e.y()),
            EntityKind::Swing(e) => (x, y) == (e.x(), e.y()),
            EntityKind::Snake(e) => (x, y) == (e.x(), e.y()),
            EntityKind::Ovis(e) => (x, y) == (e.x(), e.y()),
            EntityKind::Scorpy(e) => (x, y) == (e.x(), e.y()),
            EntityKind::Golem(e) => (x, y) == (e.x(), e.y()),
            EntityKind::Crawler(e) => (x, y) == (e.x(), e.y()),
        }
    }

    pub fn is_harmful(&self) -> bool {
        match self {
            EntityKind::OnyxStone(e) => e.is_harmful(),
            EntityKind::Fire(e) => e.is_harmful(),
            EntityKind::Swing(e) => e.is_harmful(),
            EntityKind::Snake(e) => e.is_harmful(),
            EntityKind::Ovis(e) => e.is_harmful(),
            EntityKind::Scorpy(e) => e.is_harmful(),
            EntityKind::Golem(e) => e.is_harmful(),
            EntityKind::Crawler(e) => e.is_harmful(),
        }
    }

    pub fn damage(&self) -> u8 {
        match self {
            EntityKind::OnyxStone(e) => e.damage(),
            EntityKind::Fire(e) => e.damage(),
            EntityKind::Swing(e) => e.damage(),
            EntityKind::Snake(e) => e.damage(),
            EntityKind::Ovis(e) => e.damage(),
            EntityKind::Scorpy(e) => e.damage(),
            EntityKind::Golem(e) => e.damage(),
            EntityKind::Crawler(e) => e.damage(),
        }
    }

    pub fn name<'a>(&self) -> &'a str {
        match self {
            EntityKind::OnyxStone(e) => e.name(),
            EntityKind::Fire(e) => e.name(),
            EntityKind::Swing(e) => e.name(),
            EntityKind::Snake(e) => e.name(),
            EntityKind::Ovis(e) => e.name(),
            EntityKind::Scorpy(e) => e.name(),
            EntityKind::Golem(e) => e.name(),
            EntityKind::Crawler(e) => e.name(),
        }
    }
}

pub trait Entity<'a> {
    fn x(&self) -> i64;
    fn y(&self) -> i64;
    fn go(&mut self, x: i64, y: i64);
    fn shape(&self) -> Span<'a>;
    fn on_tick(&mut self);
    fn on_action(&self, player: &mut Player, game: &Game) -> Action;
    fn is_dead(&self) -> bool;
    fn hurt(&mut self, amount: u8);
    fn heal(&mut self, amount: u8);
    fn looking(&mut self) -> Direction;
    fn is_harmful(&self) -> bool;
    fn damage(&self) -> u8;
    fn name<'b>(&self) -> &'b str;
    
    fn looking_at(&mut self) -> (i64, i64, Direction) {
        match self.looking() {
            Direction::Up => (self.x(), self.y() + 1, Direction::Up),
            Direction::Down => (self.x(), self.y() - 1, Direction::Down),
            Direction::Left => (self.x() - 1, self.y(), Direction::Left),
            Direction::Right => (self.x() + 1, self.y(), Direction::Right),
        }
    }
}
