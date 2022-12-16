use crossterm::event::{self, Event, KeyCode};
use perlin2d::PerlinNoise2D;
use tui::{
    Frame,
    symbols,
    Terminal,
    backend::Backend,
    style::{Style, Color},
    layout::{Layout, Constraint},
    widgets::{Block, Borders, Paragraph, canvas::{Canvas, Context}, Gauge}, text::Span,
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
const CHUNK_SIZE: i32 = 16;

enum Terrain {
    Water,
    Grass,
    Stone
}

impl Terrain {
    pub fn color(&self) -> Color {
        match self {
            Terrain::Water => Color::Cyan,
            Terrain::Grass => Color::Green,
            Terrain::Stone => Color::Gray,
        }
    }
}

struct Chunk(i32, i32, Vec<(Terrain, Option<BlockKind>)>);

impl Chunk {
    fn new(col: i32, row: i32, perlin: &PerlinNoise2D) -> Self {
        let mut terrain = Vec::new();
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                let x = (col*CHUNK_SIZE + i) as f64;
                let y = (row*CHUNK_SIZE + j) as f64;
                let value = perlin.get_noise(x, y);
                if value >= 0.9 {
                    terrain.push((Terrain::Stone, None))
                } else if value >= -1.0 {
                    terrain.push((Terrain::Grass, None))
                } else {
                    terrain.push((Terrain::Water, None))
                }
            }
        }
        Self(col, row, terrain)
    }

    fn draw(&self, ctx: &mut Context) {
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                let x = (self.0*CHUNK_SIZE + i) as f64;
                let y = (self.1*CHUNK_SIZE + j) as f64;
                let color = self[(i as usize, j as usize)].0.color();
                ctx.print(x, y, Span::styled("~", Style::default().bg(color).fg(Color::LightBlue)))
            }
        }
    }
}

impl Index<(usize, usize)> for Chunk {
    type Output = (Terrain, Option<BlockKind>);

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.2[i * 16 + j]
    }
}

impl IndexMut<(usize, usize)> for Chunk {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.2[i * 16 + j]
    }
}

pub struct Game {
    should_quit: bool,
    player: Player,
    entities: Vec<EntityKind>,
    loaded_chunks: Vec<Chunk>,
    offset: (f64, f64),
    x_bounds: f64,
    y_bounds: f64,
    perlin: PerlinNoise2D
}

impl<'a> Game {
    pub fn new() -> Self {
        let mut player = Player::new(0.0, 0.0);
        player.pick_up(ItemKind::FT(FlameThrower));

        let perlin = PerlinNoise2D::new(
            6,
            1.0,
            0.5,
            1.0,
            2.0,
            (100.0, 100.0),
            0.5,
            10
        );
        Game {
            should_quit: false,
            player,
            entities: Vec::new(),
            loaded_chunks: Vec::new(),
            offset: (0.0, 0.0),
            x_bounds: 0.0,
            y_bounds: 0.0,
            perlin
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
        let x = self.player.x() - self.offset.0;
        let y = self.player.y() - self.offset.1;
        let w = self.x_bounds - 10.0;
        let h = self.y_bounds - 5.0;

        if x < -w {
            self.offset.0 += w+x;
        } else if x > w {
            self.offset.0 += x-w;
        }
        
        if y < -h {
            self.offset.1 += y+h;
        } else if y > h {
            self.offset.1 += y-h;
        }

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

    fn set_bounds(&mut self, w: f64, h: f64) {
        self.x_bounds = w;
        self.y_bounds = h;
    }

    fn update_chunks(&mut self) {
        let n = self.x_bounds as i32 / CHUNK_SIZE + 2;
        let m = self.y_bounds as i32 / CHUNK_SIZE + 2;
        let c = self.offset.0 as i32 / CHUNK_SIZE;
        let r = self.offset.1 as i32 / CHUNK_SIZE;

        for i in -n..=n {
            for j in -m..=m {
                let mut found = false;
                for chunk in &self.loaded_chunks {
                    if chunk.0 == c+i && chunk.1 == r+j {
                        found = true;
                        break
                    }
                }
                if !found {
                    self.loaded_chunks.push(Chunk::new(c+i, r+j, &self.perlin))
                }
            }
        }
    }
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, mut game: Game) -> io::Result<()> {
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();
    loop {

        // time update \\
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // input handler \\
        if crossterm::event::poll(tick_rate)? {
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
            game.update_chunks();
            last_tick = Instant::now();
        }
        terminal.draw(|frame| draw(frame, &mut game))?;

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
    let w = (vchunks[1].width - 3) as f64 / 2.0;
    let h = (vchunks[1].height - 3) as f64 / 2.0;
    let x_bounds = [-w+game.offset.0, w+game.offset.0];
    let y_bounds = [-h+game.offset.1, h+game.offset.1];
    game.set_bounds(w, h);

    let player = &game.player;
    let canvas = Canvas::default()
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .block(Block::default().title(TITLE).borders(Borders::ALL))
        .marker(symbols::Marker::Block)
        .paint(|ctx| {
            for chunck in &game.loaded_chunks {
                chunck.draw(ctx);
            }
            player.draw(ctx);
            for entity in &game.entities {
                entity.draw(ctx)
            }
        });
    frame.render_widget(canvas, vchunks[1]);
}

