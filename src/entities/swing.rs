use tui::{    
    style::{Color, Style},
    text::Span, widgets::canvas::Context,
};
use crate::entities::{Direction, Entity};

pub struct Swing {
    x: f64,
    y: f64,
    looking: Direction,
    life: u8,
    damage: u8,
}

impl<'a> Swing {
    pub fn new(x: f64, y: f64, direction: Direction) -> Self {
        Self {
            x,
            y,
            looking: direction,
            life: 1,
            damage: 10,
        }
    }
}

impl<'a> Entity<'a> for Swing {
    fn shape(&self) -> Span<'a> {
        match self.looking {
            Direction::Up => Span::styled("-", Style::default().fg(Color::Yellow)),
            Direction::Down => Span::styled("-", Style::default().fg(Color::Yellow)),
            Direction::Left => Span::styled("|", Style::default().fg(Color::Yellow)),
            Direction::Right => Span::styled("|", Style::default().fg(Color::Yellow)),
        }
    }

    fn draw<'b>(&'a self, ctx: &mut Context<'b>) {
        ctx.print(self.x, self.y, self.shape())
    }

    fn on_tick(&mut self) {
        self.life = 0;
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
