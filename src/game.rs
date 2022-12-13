use crossterm::event::{self, Event, KeyCode};
use tui::{
    Frame,
    symbols,
    Terminal,
    backend::Backend,
    style::{Style, Color},
    layout::{Layout, Constraint},
    widgets::{Block, Borders, Paragraph, canvas::Canvas, Gauge},
};
use std::{
    io,
    time::{Duration, Instant}, ops::{Index, IndexMut}
};
use crate::{entities::{
    EntityKind,
    player::Player, Direction
}, items::{flamethrower::FlameThrower, ItemKind}, blocks::BlockKind};

const TITLE: &str = "Game";

struct Chunck {
    size: u8,
    blocks: Vec<Option<BlockKind>>
}

impl Chunck {
    pub fn new(init: bool) -> Self {
        Self {
            size: 16,
            blocks: vec![None; 256]
        }
    }
}

impl Index<(usize, usize)> for Chunck {
    type Output = Option<BlockKind>;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.blocks[i * 16 + j]
    }
}

impl IndexMut<(usize, usize)> for Chunck {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.blocks[i * 16 + j]
    }
}

pub struct Game {
    should_quit: bool,
    player: Player,
    entities: Vec<EntityKind>
}

impl<'a> Game {
    pub fn new() -> Self {
        let mut player = Player::new(75.0, 25.0);
        player.pick_up(ItemKind::FT(FlameThrower));
        Game {
            should_quit: false,
            player,
            entities: Vec::new(),
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            ' ' => {
                if let Some(entity) = self.player.on_space() {
                    self.entities.push(entity);
                }
            }
            'q' => self.should_quit = true,
            _ => {}
        }
    }

    pub fn on_escape(&mut self) {
        self.should_quit = true;
    }

    pub fn on_tick(&mut self) {
        self.player.on_tick();
        let mut fire_generated = Vec::new();

        for entity in &mut self.entities {
            if let EntityKind::Fire(fire) = entity {
                fire_generated.extend(fire.spread());
            }
            entity.on_tick();
        }
        self.entities.retain(|e| !e.is_dead());
        for fire in fire_generated {
            self.entities.push(fire);
        }
    }
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, mut game: Game) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| draw(frame, &mut game))?;

        // time update \\
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // input handler \\
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => game.on_key(c),
                    KeyCode::Esc => game.on_escape(),
                    KeyCode::Up => game.player.go(Direction::Up, &game.entities),
                    KeyCode::Down => game.player.go(Direction::Down, &game.entities),
                    KeyCode::Left => game.player.go(Direction::Left, &game.entities),
                    KeyCode::Right => game.player.go(Direction::Right, &game.entities),
                    _ => {}
                }
            }
        }

        // game update \\
        if last_tick.elapsed() >= tick_rate {
            game.on_tick();
            last_tick = Instant::now();
        }

        if game.should_quit {
            return Ok(());
        }
    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, game: &mut Game) {
    let vchunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(2)])
        .split(frame.size());

    let hchunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(tui::layout::Direction::Horizontal)
        .split(vchunks[0]);

    // controls information \\
    let paragraph = Paragraph::new("")
        .block(Block::default().title(TITLE).borders(Borders::ALL));
    frame.render_widget(paragraph, hchunks[0]);

    let lifebar = Gauge::default()
        .block(Block::default().title("[Life]").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(game.player.life_ratio());
    frame.render_widget(lifebar, hchunks[1]);

    // canvas \\
    let player = &game.player;
    let x_bounds = [0.0, (vchunks[1].width-2) as f64];
    let y_bounds = [0.0, (vchunks[1].height-2) as f64];
    let canvas = Canvas::default()
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .block(Block::default().title(TITLE).borders(Borders::ALL))
        .marker(symbols::Marker::Block)
        .background_color(Color::Green)
        .paint(|ctx| {
            player.draw(ctx);
            for entity in &game.entities {
                entity.draw(ctx)
            }
        });
    frame.render_widget(canvas, vchunks[1]);
}

