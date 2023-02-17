use rand::Rng;
use tui::{    
    style::{Color, Style},
    text::Span, widgets::canvas::Context,
};
use crate::{entities::{Direction, Entity}, game::Game};

use super::{EntityKind, player::Player, Action};

pub struct Fire {
    x: f64,
    y: f64,
    looking: Direction,
    life: u8,
    max_life: u32,
    damage: u8,
}

impl<'a> Fire {
    pub fn new(x: f64, y: f64, direction: Direction) -> Self {
        Fire {
            x,
            y,
            looking: direction,
            life: 10,
            max_life: 10,
            damage: 5,
        }
    }

    pub fn spreaded(x: f64, y: f64, direction: Direction, life: u8) -> EntityKind {
        EntityKind::Fire(Fire {
            x,
            y,
            looking: direction,
            life,
            max_life: 10,
            damage: 5,
        })
    }
}

impl<'a> Entity<'a> for Fire {
    fn shape(&self) -> Span<'a> {
        let color = match self.life {
            10 => Color::White,
            9 => Color::Rgb(251, 228, 26),
            8 => Color::Rgb(247, 138, 7),
            7 => Color::Rgb(191, 66, 4),
            _ => Color::Rgb(110, 39, 4),
        };
        Span::styled("@", Style::default().fg(color))
    }

    fn draw<'b>(&'a self, ctx: &mut Context<'b>) {
        ctx.print(self.x, self.y, self.shape())
    }

    fn go(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    fn on_tick(&mut self) {
        self.life = 0;
    }

    fn on_action(&self, player: &mut Player, game: &Game) -> Action {
        let mut fire = Vec::new();
        let mut rng = rand::thread_rng();
        let side_rng = if self.life >= 6 {
            (self.life - 6) as u32
        } else { 0 };
        match self.looking {
            Direction::Up => {
                if rng.gen_ratio(self.life as u32, self.max_life) {
                    fire.push(Fire::spreaded(self.x, self.y+1.0, self.looking.to_owned(), self.life-1));
                }
                if rng.gen_ratio(side_rng, self.max_life) {
                    fire.push(Fire::spreaded(self.x-1.0, self.y, self.looking.to_owned(), self.life-5));
                }
                if rng.gen_ratio(side_rng, self.max_life) {
                    fire.push(Fire::spreaded(self.x+1.0, self.y, self.looking.to_owned(), self.life-5));
                }
            },
            Direction::Down => {
                if rng.gen_ratio(self.life as u32, self.max_life) {
                    fire.push(Fire::spreaded(self.x, self.y-1.0, self.looking.to_owned(), self.life-1));
                }
                if rng.gen_ratio(side_rng, self.max_life) {
                    fire.push(Fire::spreaded(self.x-1.0, self.y, self.looking.to_owned(), self.life-5));
                }
                if rng.gen_ratio(side_rng, self.max_life) {
                    fire.push(Fire::spreaded(self.x+1.0, self.y, self.looking.to_owned(), self.life-5));
                }
            },
            Direction::Left => {
                if rng.gen_ratio(self.life as u32, self.max_life) {
                    fire.push(Fire::spreaded(self.x-1.0, self.y, self.looking.to_owned(), self.life-1));
                }
                if rng.gen_ratio(side_rng, self.max_life) {
                    fire.push(Fire::spreaded(self.x, self.y-1.0, self.looking.to_owned(), self.life-5));
                }
                if rng.gen_ratio(side_rng, self.max_life) {
                    fire.push(Fire::spreaded(self.x, self.y+1.0, self.looking.to_owned(), self.life-5));
                }
            },
            Direction::Right => {
                if rng.gen_ratio(self.life as u32, self.max_life) {
                    fire.push(Fire::spreaded(self.x+1.0, self.y, self.looking.to_owned(), self.life-1));
                }
                if rng.gen_ratio(side_rng, self.max_life) {
                    fire.push(Fire::spreaded(self.x, self.y-1.0, self.looking.to_owned(), self.life-5));
                }
                if rng.gen_ratio(side_rng, self.max_life) {
                    fire.push(Fire::spreaded(self.x, self.y+1.0, self.looking.to_owned(), self.life-5));
                }
            },
        };
        Action::Spawn(fire)
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

    fn collide(&self, _x: f64, _y: f64) -> bool {
        false
    }
}
