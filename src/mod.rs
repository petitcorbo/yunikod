use tui::{    
    style::{Color, Style},
    text::Span, widgets::canvas::Context,
};

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum EntityKind {
    Bullet(Bullet)
}

impl EntityKind {
    pub fn on_tick(&mut self) {
        match self {
            EntityKind::Bullet(b) => b.on_tick(),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        match self {
            EntityKind::Bullet(b) => b.draw(ctx),
        }
    }
}

pub trait Entity {
    fn go(direction: Direction) {
        match direction {
            Direction::Up => {},
            Direction::Down => {},
            Direction::Left => {},
            Direction::Right => {},
        }
    }

    fn on_tick(&self);
    fn draw(&self, ctx: Context);
}
