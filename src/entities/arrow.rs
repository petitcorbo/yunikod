use tui::{    
    style::{Color, Style},
    text::Span,
};
use crate::{entities::{Direction, Entity}, game::Game};

use super::{player::Player, Action};

pub struct Arrow {
    x: i64,
    y: i64,
    looking: Direction,
    life: u8,
    damage: u8,
}

impl<'a> Arrow {
    pub fn new(x: i64, y: i64, direction: Direction, damage: u8) -> Self {
        Self {
            x,
            y,
            looking: direction,
            life: 50,
            damage,
        }
    }
}

impl<'a> Entity<'a> for Arrow {
    fn name<'b>(&self) -> &'b str {
        "swing"
    }

    fn shape(&self) -> Span<'a> {
        match self.looking {
            Direction::Up => Span::styled("|", Style::default().fg(Color::LightYellow)),
            Direction::Down => Span::styled("|", Style::default().fg(Color::LightYellow)),
            Direction::Left => Span::styled("-", Style::default().fg(Color::LightYellow)),
            Direction::Right => Span::styled("-", Style::default().fg(Color::LightYellow)),
        }
    }

    fn go(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
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

    fn on_action(&self, _player: &mut Player, game: &Game) -> Action {
        if let Some(entity_id) = game.get_entity_id(self.x, self.y) {
            Action::Attack(entity_id, self.damage)
        } else {
            Action::Nothing
        }
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
