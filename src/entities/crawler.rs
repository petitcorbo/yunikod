use rand::Rng;
use tui::{    
    style::{Color, Style},
    text::Span,
};
use crate::{entities::{Direction, Entity}, game::Game};

use super::{player::Player, Action};
use rust_i18n::t;
rust_i18n::i18n!("locales");

pub struct Crawler {
    x: i64,
    y: i64,
    looking: Direction,
    life: u8,
    damage: u8,
    frame: u8,
    immunity: u8,
    until_next_step: u8,
}

impl<'a> Crawler {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
            looking: Direction::Up,
            life: 5,
            damage: 8,
            frame: 0,
            immunity: 0,
            until_next_step: 10
        }
    }
}

impl<'a> Entity<'a> for Crawler {
    fn name(&self,lang: String) -> String {
       rust_i18n::set_locale(&lang); //set language
       t!("game.entity.crawler")
    } 

    fn shape(&self) -> Span<'a> {
        let color = if self.immunity == 0 {
            Color::Black
        } else  {
            Color::Red
        };
        if self.frame < 10 {
            Span::styled("X", Style::default().fg(color))
        } else {
            Span::styled("x", Style::default().fg(color))
        }
    }

    fn go(&mut self, x: i64, y: i64) {
        if self.until_next_step > 0 {
            self.until_next_step -= 1;
        } else {
            self.x = x;
            self.y = y;
            self.until_next_step = 10;
        }
    }

    fn on_tick(&mut self) {
        self.frame = (self.frame + 1) % 20;
        if self.immunity > 0 { self.immunity -= 1 }
    }

    fn on_action(&self, player: &mut Player, game: &Game) -> super::Action {
        let mut x = self.x;
        let mut y = self.y;
        let delta_x = (x - player.x()).abs();
        let delta_y = (y - player.y()).abs();

        // hurt the player if he is in range
        if (delta_x == 1 && delta_y == 0) || (delta_x == 0 && delta_y == 1) {
            player.hurt(self.damage);
            return Action::Nothing;
        }

        // try to move
        if self.until_next_step > 0 {
            return Action::Move(x, y);
        }
        // chase player if in agro zone
        if delta_x < 10 && delta_y < 10 {
            if delta_x > delta_y {
                if x > player.x() {
                    x -= 1;
                } else {
                    x += 1;
                }
            } else {
                if y > player.y() {
                    y -= 1;
                } else {
                    y += 1;
                }
            }
        } else {
            // move randomly
            match rand::thread_rng().gen_range(0..=3) {
                0 => x += 1,
                1 => x -= 1,
                2 => y += 1,
                3 => y -= 1,
                _ => {}
            }
        }
        // check if there is something already at the coordinates
        if game.is_available(x, y) && (player.x() != x || player.y() != y) {
            return Action::Move(x, y);
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

    fn hurt(&mut self, amount: u8) {
        if self.immunity == 0 {
            if self.life < amount {
                self.life = 0;
            } else {
                self.life -= amount;
            }
            self.immunity = 10
        }
    }

    fn is_harmful(&self) -> bool {
        true
    }
    
    fn damage(&self) -> u8 {
        self.damage
    }
}
