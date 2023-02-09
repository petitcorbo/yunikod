use tui::{    
    style::{Color, Style},
    text::Span, widgets::canvas::Context,
};
use crate::{entities::{Direction, Entity}, game::{Game, self}, chunk::Terrain};

use super::{player::Player, EntityKind};

pub struct Snake {
    x: f64,
    y: f64,
    looking: Direction,
    life: u8,
    damage: u8,
    frame: u8,
    immunity: u8
}

impl<'a> Snake {
    pub fn new(x: f64, y: f64, direction: Direction, damage: u8) -> Self {
        Self {
            x,
            y,
            looking: direction,
            life: 3,
            damage,
            frame: 0,
            immunity: 0
        }
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
}

impl<'a> Entity<'a> for Snake {
    fn shape(&self) -> Span<'a> {
        if self.frame < 10 {
            Span::styled("S", Style::default().fg(Color::Yellow))
        } else {
            Span::styled("s", Style::default().fg(Color::Yellow))
        }
    }

    fn draw<'b>(&'a self, ctx: &mut Context<'b>) {
        ctx.print(self.x, self.y, self.shape())
    }

    fn on_tick(&mut self, player: &mut Player, game: &Game) {
        let delta_x = self.x - player.x();
        let delta_y = self.y - player.y();
        if delta_x.abs() > delta_y.abs() {
            if delta_x.is_sign_positive() {
                self.looking = Direction::Right;
            } else {
                self.looking = Direction::Left;
            }
        } else {
            if delta_y.is_sign_positive() {
                self.looking = Direction::Up;
            } else {
                self.looking = Direction::Down;
            }
        }
    }

    fn is_dead(&self) -> bool {
        self.life == 0
    }

    fn looking(&mut self) -> Direction {
        self.looking.to_owned()
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
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
