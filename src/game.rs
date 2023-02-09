use crossterm::event::{self, Event, KeyCode};
use perlin2d::PerlinNoise2D;
use rand::{thread_rng, Rng};
use tui::{
    Frame,
    symbols,
    Terminal,
    backend::Backend,
    style::{Style, Color},
    layout::{Layout, Constraint},
    widgets::{Block, Borders, Paragraph, canvas::Canvas, Gauge, List, ListState}
};
use std::{
    io,
    time::{Duration, Instant}, borrow::BorrowMut
};
use crate::{entities::{
    EntityKind,
    player::Player, Direction, snake::Snake
}, blocks::BlockKind, chunk::{Chunk, CHUNK_SIZE, Terrain}, ui::{inventory, crafting}};

const TITLE: &str = "Yuni-Kod";

pub struct Game {
    should_quit: bool,
    entities: Vec<EntityKind>,
    loaded_chunks: Vec<Chunk>,
    offset: (f64, f64),
    x_bounds: f64,
    y_bounds: f64,
    perlin: PerlinNoise2D,
    message: String,
    message_timer: u8
}

impl<'a> Game {
    pub fn new() -> Self {

        let perlin = PerlinNoise2D::new(
            1,
            100.0,
            10.0,
            1.0,
            2.0,
            (100.0, 100.0),
            0.5,
            10
        );
        Game {
            should_quit: false,
            entities: Vec::new(),
            loaded_chunks: Vec::new(),
            offset: (0.0, 0.0),
            x_bounds: 0.0,
            y_bounds: 0.0,
            perlin,
            message: String::new(),
            message_timer: 0
        }
    }

    pub fn on_escape(&mut self) {
        self.should_quit = true;
    }

    pub fn entities(&self) -> &Vec<EntityKind> {
        &self.entities
    }

    pub fn on_tick(&mut self, mut player: &mut Player) {
        if self.message_timer > 1 {
            self.message_timer -= 1;
        } else if self.message_timer == 1 {
            self.message.clear();
        }
        let x = player.x() - self.offset.0;
        let y = player.y() - self.offset.1;
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

        if thread_rng().gen_ratio(1, 50) {
            let x = thread_rng().gen_range(-self.x_bounds..self.x_bounds);
            let y = thread_rng().gen_range(-self.y_bounds..self.y_bounds);
            let snake = Snake::new(x, y, Direction::Up, 5);
            self.entities.push(EntityKind::Snake(snake));
        }

        let mut fire_generated = Vec::new();
        for entity in &mut self.entities {
            if let EntityKind::Fire(fire) = entity {
                fire_generated.extend(fire.spread());
            }
        }

        for i in 0..self.entities.len() {
            let entity = self.entities[i];
            let e = entity.borrow_mut();copy
            e.on_tick(&mut player, &self);
        }
        self.entities.retain(|e| !e.is_dead());
        for fire in fire_generated {
            self.entities.push(fire);
        }
    }

    pub fn is_available(&self, x: f64, y: f64) -> bool {
        let mut available = true;
        for entity in &self.entities {
            if entity.collide(x, y) {
                return false;
            }
        }
        match self.get_tile(x, y) {
            Terrain::Water => available = false,
            Terrain::DeepWater => available = false,
            _ => {},
        }
        available
    }

    pub fn get_block(&mut self, x: f64, y: f64) -> Option<&mut BlockKind> {
        let x = x.floor() as i32;
        let y = y.floor() as i32;
        let mut chunk_idx = ( x / CHUNK_SIZE, y / CHUNK_SIZE );
        if x < 0 && x%CHUNK_SIZE != 0 { chunk_idx.0 -= 1 }
        if y < 0 && y%CHUNK_SIZE != 0 { chunk_idx.1 -= 1 }
        for chunk in &mut self.loaded_chunks {
            if chunk_idx == (chunk.0, chunk.1) {
                let i = x%CHUNK_SIZE;
                let j = y%CHUNK_SIZE;
                let i = ((CHUNK_SIZE+i)%CHUNK_SIZE) as usize;
                let j = ((CHUNK_SIZE+j)%CHUNK_SIZE) as usize;
                return chunk[(i, j)].1.as_mut();
            }
        }
        None
    }

    pub fn get_tile(&self, x: f64, y: f64) -> &Terrain {
        let x = x.floor() as i32;
        let y = y.floor() as i32;
        let mut chunk_idx = ( x / CHUNK_SIZE, y / CHUNK_SIZE );
        if x < 0 && x%CHUNK_SIZE != 0 { chunk_idx.0 -= 1 }
        if y < 0 && y%CHUNK_SIZE != 0 { chunk_idx.1 -= 1 }
        for chunk in &self.loaded_chunks {
            if chunk_idx == (chunk.0, chunk.1) {
                let i = x%CHUNK_SIZE;
                let j = y%CHUNK_SIZE;
                let i = ((CHUNK_SIZE+i)%CHUNK_SIZE) as usize;
                let j = ((CHUNK_SIZE+j)%CHUNK_SIZE) as usize;
                return &chunk[(i, j)].0;
            }
        }
        &Terrain::Grass
    }

    pub fn destroy_block(&mut self, x: f64, y: f64) {
        let x = x.floor() as i32;
        let y = y.floor() as i32;
        let mut chunk_idx = ( x / CHUNK_SIZE, y / CHUNK_SIZE );
        if x < 0 { chunk_idx.0 -= 1 }
        if y < 0 { chunk_idx.1 -= 1 }
        for chunk in &mut self.loaded_chunks {
            if chunk_idx == (chunk.0, chunk.1) {
                let i = x%CHUNK_SIZE;
                let j = y%CHUNK_SIZE;
                let i = ((CHUNK_SIZE+i)%CHUNK_SIZE) as usize;
                let j = ((CHUNK_SIZE+j)%CHUNK_SIZE) as usize;
                chunk[(i, j)].1 = None;
            }
        }
    }

    pub fn message(&self) -> String {
        self.message.to_owned()
    }

    pub fn set_message(&mut self, s: String) {
        self.message = s;
        self.message_timer = 10;
    }

    fn set_bounds(&mut self, w: f64, h: f64) {
        self.x_bounds = w;
        self.y_bounds = h;
    }

    pub fn update_chunks(&mut self) {
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

fn on_key<B: Backend>(terminal: &mut Terminal<B>, game: &mut Game, player: &mut Player, c: char) {
    match c {
        ' ' => {
            if let Some(entity) = player.on_space(game) {
                game.entities.push(entity);
            }
        }
        'q' => game.should_quit = true,
        'i' => {inventory::run(terminal, game, player);},
        'c' => {crafting::run(terminal, game, player);},
        _ => {}
    }
}


pub fn run<B: Backend>(terminal: &mut Terminal<B>, mut game: Game, mut player: Player) -> io::Result<()> {
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();
    loop {

        // time update \\
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // input handler \\
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => on_key(terminal, &mut game, &mut player, c),
                    KeyCode::Esc => game.on_escape(),
                    KeyCode::Up => player.on_arrow(&key, Direction::Up),
                    KeyCode::Down => player.on_arrow(&key, Direction::Down),
                    KeyCode::Left => player.on_arrow(&key, Direction::Left),
                    KeyCode::Right => player.on_arrow(&key, Direction::Right),
                    _ => {}
                }
            }
        }

        // game update \\
        if last_tick.elapsed() >= tick_rate {
            player.on_tick(&mut game);
            player.moving(false);
            game.on_tick(&mut player);
            game.update_chunks();
            last_tick = Instant::now();
        }
        terminal.draw(|frame| draw(frame, &mut game, &mut player))?;

        if game.should_quit {
            return Ok(());
        }
    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, game: &mut Game, player: &mut Player) {
    let vchunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(2), Constraint::Length(3)])
        .split(frame.size());

    let hchunks0 = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(tui::layout::Direction::Horizontal)
        .split(vchunks[0]);

    // controls information \\
    let text = format!("x:{} y:{} p:{}", player.x(), player.y(), game.perlin.get_noise(player.x(), player.y()));
    let paragraph = Paragraph::new(text)
        .block(Block::default().title(TITLE).borders(Borders::ALL));
    frame.render_widget(paragraph, hchunks0[0]);

    let lifebar = Gauge::default()
        .block(Block::default().title("[Life]").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(player.life_ratio());
    frame.render_widget(lifebar, hchunks0[1]);

    let hchunks1 = Layout::default()
        .constraints([Constraint::Min(3), Constraint::Length(4)])
        .direction(tui::layout::Direction::Horizontal)
        .split(vchunks[1]);

    // canvas \\
    let w = (hchunks1[0].width - 3) as f64 / 2.0;
    let h = (hchunks1[0].height - 3) as f64 / 2.0;
    let x_bounds = [-w+game.offset.0, w+game.offset.0];
    let y_bounds = [-h+game.offset.1, h+game.offset.1];
    game.set_bounds(w, h);

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
    frame.render_widget(canvas, hchunks1[0]);

    let mut list_state = ListState::default();
    list_state.select(Some(player.using()));
    let list = List::new(player.inventory().to_item_list())
        .block(Block::default().borders(Borders::ALL))
        .highlight_symbol(">");
    frame.render_stateful_widget(list, hchunks1[1], &mut list_state);

    let para_message = Paragraph::new(game.message.clone())
        .block(Block::default().title(TITLE).borders(Borders::ALL));
    frame.render_widget(para_message, vchunks[2]);
}

