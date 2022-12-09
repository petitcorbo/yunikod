use crossterm::event::{
    self, Event, KeyCode
};
use tui::{
    backend::Backend,
    Terminal,
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
    layout::{Layout, Constraint},
    symbols,
    Frame,
};
use std::{
    io,
    time::{Duration, Instant}
};
use crate::{entities::{
    Entity, EntityKind,
    player::Player
}, items::{gun::Gun, ItemKind}};

const TITLE: &str = "Game";

pub struct Game {
    should_quit: bool,
    player: Player,
    entities: Vec<EntityKind>
}

impl<'a> Game {
    pub fn new() -> Self {
        let mut player = Player::new(75.0, 25.0);
        player.pick_up(ItemKind::Gun(Gun));
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

        for entity in &mut self.entities {
            entity.on_tick();
        }
        self.entities.retain(|e| !e.is_dead());
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
                    KeyCode::Up => game.player.on_up(),
                    KeyCode::Down => game.player.on_down(),
                    KeyCode::Left => game.player.on_left(),
                    KeyCode::Right => game.player.on_right(),
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
        .constraints([Constraint::Length(3), Constraint::Min(2)])
        .split(vchunks[0]);

    // controls information \\
    let paragraph = Paragraph::new("ok")
        .block(Block::default().title(TITLE).borders(Borders::ALL));
    frame.render_widget(paragraph, hchunks[0]);

    // canvas \\
    let player = &game.player;
    let x_bounds = [0.0, (vchunks[1].width-2) as f64];
    let y_bounds = [0.0, (vchunks[1].height-2) as f64];
    let canvas = Canvas::default()
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .block(Block::default().title(TITLE).borders(Borders::ALL))
        .marker(symbols::Marker::Block)
        .paint(|ctx| {
            player.draw(ctx);
            for entity in &game.entities {
                entity.draw(ctx)
            }
        });
    frame.render_widget(canvas, vchunks[1]);
}

