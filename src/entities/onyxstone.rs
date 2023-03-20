use tui::{    
    style::{Color, Style},
    text::Span,
};
use crate::{entities::{Direction, Entity}, game::Game};

use super::player::Player;

pub struct OnyxStone {
    x: i64,
    y: i64,
    looking: Direction,
    life: u8,
    damage: u8,
}

impl<'a> OnyxStone {
    pub fn new(x: i64, y: i64, direction: Direction) -> Self {
        Self {
            x,
            y,
            looking: direction,
            life: 20,
            damage: 10,
        }
    }
}

impl<'a> Entity<'a> for OnyxStone {
    fn name<'b>(&self) -> &'b str {
        "onyx stone"
    }

    fn shape(&self) -> Span<'a> {
        Span::styled("*", Style::default().fg(Color::Yellow))
    }

    fn on_tick(&mut self) {
        match self.looking {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
        if !self.is_dead() {
            self.life -= 1;
        }
    }

    fn go(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
    }

    fn on_action(&self, _player: &mut Player, _game: &Game) -> super::Action {
        super::Action::Nothing
    }

    fn is_dead(&self) -> bool {
        self.life == 0
    }

    fn looking(&mut self) -> Direction {
        self.looking.to_owned()
    }

    fn x(&self) -> i64 {
        self.x
    }

    fn y(&self) -> i64 {
        self.y
    }

    fn heal(&mut self, _amount: u8) {}
    fn hurt(&mut self, _amount: u8) {}

    fn is_harmful(&self) -> bool {
        true
    }
    
    fn damage(&self) -> u8 {
        self.damage
    }
}
