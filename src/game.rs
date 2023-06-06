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
    time::{Duration, Instant}
};
use crate::{entities::{
    EntityKind,
    player::Player, Direction, Action,
}, blocks::BlockKind, chunk::{Chunk, CHUNK_SIZE, Terrain}, ui::{inventory, crafting, map, game_over}};
use locales::t;

const TITLE: &str = "Yuni-Kod";

pub struct Game {
    should_quit: bool,
    entities: Vec<EntityKind>,
    loaded_chunks: Vec<Chunk>,
    unused_chunks: Vec<Chunk>,
    offset: (i64, i64),
    x_bounds: i64,
    y_bounds: i64,
    perlin: PerlinNoise2D,
    message: String,
    message_timer: u8,
    pub language: String
}

impl<'a> Game {
    pub fn new() -> Self {

        let perlin = PerlinNoise2D::new(
            1,
            100.0,
            5.0, // 5.0
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
            unused_chunks: Vec::new(),
            offset: (0, 0),
            x_bounds: 0,
            y_bounds: 0,
            perlin,
            message: String::new(),
            message_timer: 0,
            language: String::new()
        }
    }

    pub fn on_escape(&mut self) {
        self.should_quit = true;
    }

    pub fn entities(&self) -> &Vec<EntityKind> {
        &self.entities
    }

    pub fn mut_entities(&mut self) -> &mut Vec<EntityKind> {
        &mut self.entities
    }

    pub fn loaded_chunks(&self) -> &Vec<Chunk> {
        &self.loaded_chunks
    }

    pub fn unused_chunks(&self) -> &Vec<Chunk> {
        &self.unused_chunks
    }

    pub fn perlin(&self) -> &PerlinNoise2D {
        &self.perlin
    }

    pub fn on_tick(&mut self, player: &mut Player, lang: String) {
        // update message
        if self.message_timer > 1 {
            self.message_timer -= 1;
        } else if self.message_timer == 1 {
            self.message.clear();
        }

        // update camera
        let x = player.x() - self.offset.0;
        let y = player.y() - self.offset.1;
        let w = self.x_bounds / 3;
        let h = self.y_bounds / 3;

        if x < -w {
            self.offset.0 += x+w;
        } else if x > w {
            self.offset.0 += x-w;
        }
        
        if y < -h {
            self.offset.1 += y+h;
        } else if y > h {
            self.offset.1 += y-h;
        }

        // spawn entities
        if thread_rng().gen_ratio(1, 50) {
            let x_range = (-self.x_bounds+self.offset.0)..(self.x_bounds+self.offset.0);
            let y_range = (-self.y_bounds+self.offset.1)..(self.y_bounds+self.offset.1);
            let x = thread_rng().gen_range(x_range) as i64;
            let y = thread_rng().gen_range(y_range) as i64;
            if let Some(entity) = self.get_tile(x, y).random_entity(x, y) {
                self.entities.push(entity);
            }
        }

        // update entities
        for i in 0..self.entities.len() {
            let action = self.entities[i].on_action(player, self);
            match action {
                Action::Move(x, y) => self.entities[i].go(x, y),
                Action::Spawn(mut entities) => self.entities.append(&mut entities),
                Action::Attack(id, damage) => {
                    let target = &mut self.entities[id];
                    target.hurt(damage);
                    self.set_message(format!("{} {} {} {}", id,t!("game.msg.took", lang), damage,t!("game.msg.damage", lang)));
                },
                Action::Nothing => {},
            };
            self.entities[i].on_tick();
        }
        
        // destroy dead entities
        self.entities.retain(|e| !e.is_dead());
    }

    pub fn is_available(&self, x: i64, y: i64) -> bool {
        self.get_entity_id(x, y).is_none() &&
        self.get_block(x, y).is_none() &&
        match self.get_tile(x, y) {
            Terrain::Water => false,
            Terrain::DeepWater => false,
            _ => true,
        }
    }

    pub fn get_entity_id(&self, x: i64, y: i64) -> Option<usize> {
        for i in 0..self.entities.len() {
            if self.entities[i].collide(x, y) {
                return Some(i);
            }
        }
        None
    }

    pub fn get_block(&self, x: i64, y: i64) -> Option<&BlockKind> {
        let mut chunk_idx = ( x / CHUNK_SIZE, y / CHUNK_SIZE );
        if x < 0 && x%CHUNK_SIZE != 0 { chunk_idx.0 -= 1 }
        if y < 0 && y%CHUNK_SIZE != 0 { chunk_idx.1 -= 1 }
        for chunk in &self.loaded_chunks {
            if chunk_idx == (chunk.0, chunk.1) {
                let i = x%CHUNK_SIZE;
                let j = y%CHUNK_SIZE;
                let i = ((CHUNK_SIZE+i)%CHUNK_SIZE) as usize;
                let j = ((CHUNK_SIZE+j)%CHUNK_SIZE) as usize;
                return chunk[(i, j)].1.as_ref();
            }
        }
        None
    }

    pub fn get_mut_block(&mut self, x: i64, y: i64) -> Option<&mut BlockKind> {
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

    pub fn get_tile(&self, x: i64, y: i64) -> &Terrain {
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

    pub fn destroy_block(&mut self, x: i64, y: i64) {
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
                return;
            }
        }
    }
    pub fn get_lang(&self)-> String{
      self.language.to_owned()
    }
    pub fn message(&self) -> String {
        self.message.to_owned()
    }

    pub fn set_message(&mut self, s: String) {
        self.message = s;
        self.message_timer = 20;
    }

    fn set_bounds(&mut self, w: i64, h: i64) {
        self.x_bounds = w;
        self.y_bounds = h;
    }

    pub fn update_chunks(&mut self) {
        let x0 = (-self.x_bounds + self.offset.0) as i64;
        let x1 = ( self.x_bounds + self.offset.0) as i64;
        let y0 = (-self.y_bounds + self.offset.1) as i64;
        let y1 = ( self.y_bounds + self.offset.1) as i64;
        let mut c0 = ( x0 / CHUNK_SIZE, y0 / CHUNK_SIZE );
        if x0 < 0 { c0.0 -= 1 }
        if y0 < 0 { c0.1 -= 1 }
        let mut c1 = ( x1 / CHUNK_SIZE + 1, y1 / CHUNK_SIZE + 1);
        if x1 < 0 { c1.0 -= 1 }
        if y1 < 0 { c1.1 -= 1 }
        
        // load chunks
        for i in c0.0..=c1.0 {
            for j in c0.1..=c1.1 {
                let mut found = false;
                for chunk in &self.loaded_chunks {
                    if chunk.0 == i && chunk.1 == j {
                        found = true;
                        break
                    }
                }
                if !found { // check in unused chunks
                    for idx in 0..self.unused_chunks.len() {
                        if self.unused_chunks[idx].0 == i && self.unused_chunks[idx].1 == j {
                            self.loaded_chunks.push(self.unused_chunks.swap_remove(idx));
                            found = true;
                            break
                        }
                    }
                }
                if !found { // create chunk
                    self.loaded_chunks.push(Chunk::new(i, j, &self.perlin))
                }
            }
        }

        // unload chunks
        let n_range = c0.0..=c1.0;
        let m_range = c0.1..=c1.1;
        let mut idx = 0;
        while idx < self.loaded_chunks.len() {
            if !n_range.contains(&(self.loaded_chunks[idx].0)) || !m_range.contains(&(self.loaded_chunks[idx].1)) {
                self.unused_chunks.push(self.loaded_chunks.swap_remove(idx));
            } else { idx += 1 }
        }
    }
}

fn on_key<B: Backend>(terminal: &mut Terminal<B>, game: &mut Game, player: &mut Player, c: char, lang: &mut String) {
    match c {
        ' ' => {
            if let Some(entity) = player.on_space(game) {
                game.entities.push(entity);
            }
        }
        'q' => game.should_quit = true,
        'i' => {launch_tab(terminal, game, player, 1, lang);},
        'c' => {launch_tab(terminal, game, player, 2, lang);},
        'm' => {launch_tab(terminal, game, player, 3, lang);},
        _ => {}
    }
}

fn launch_tab<B: Backend>(terminal: &mut Terminal<B>, game: &mut Game, player: &mut Player, mut n: u8, lang: &mut String) {
    while n != 0 {
        n = match n {
            1 => inventory::run(terminal, game, player, lang).unwrap(),
            2 => crafting::run(terminal, game, player, lang).unwrap(),
            3 => map::run(terminal, game, player,lang).unwrap(),
            _ => 0
        }
    }
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, mut game: Game, mut player: Player, lang: &mut String) -> io::Result<()> {
    game.language = lang.to_string();
    player.language = lang.to_string();
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
                    KeyCode::Char(c) => on_key(terminal, &mut game, &mut player, c,lang),
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
            if player.is_dead(){
              let end = game_over::run(terminal,lang);
              if let Err(error) = end{println!("{error}");}
            }
            player.on_tick(&mut game);
            player.moving(false);
            game.on_tick(&mut player, lang.to_string());
            game.update_chunks();
            last_tick = Instant::now();
        }
        terminal.draw(|frame| draw(frame, &mut game, &mut player, lang.to_string()))?;

        if game.should_quit {
            return Ok(());
        }
    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, game: &mut Game, player: &mut Player,lang: String) {
    let vchunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(2), Constraint::Length(3)])
        .split(frame.size());

    let hchunks0 = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(tui::layout::Direction::Horizontal)
        .split(vchunks[0]);

    // controls information \\
    //let text = format!("x:{} y:{} p:{}", player.x(), player.y(), game.perlin.get_noise(player.x() as f64, player.y() as f64));
    let text = format!("{} {} | {} {} | x:{} | y:{}",t!("game.ui.ldchunks",lang).as_str(), game.loaded_chunks.len(),t!("game.ui.unchunks",lang).as_str(),game.unused_chunks.len(), player.x(), player.y());
    let paragraph = Paragraph::new(text)
        .block(Block::default().title(TITLE).borders(Borders::ALL));
    frame.render_widget(paragraph, hchunks0[0]);

    let lifebar = Gauge::default()
        .block(Block::default().title(t!("game.ui.life",lang)).borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(player.life_ratio());
    frame.render_widget(lifebar, hchunks0[1]);

    let hchunks1 = Layout::default()
        .constraints([Constraint::Min(3), Constraint::Length(4)])
        .direction(tui::layout::Direction::Horizontal)
        .split(vchunks[1]);

    // canvas \\
    let w = (hchunks1[0].width as i64 - 3) / 2;
    let h = (hchunks1[0].height as i64 - 3) / 2;
    let x_bounds = [(-w+game.offset.0) as f64, (w+game.offset.0) as f64];
    let y_bounds = [(-h+game.offset.1) as f64, (h+game.offset.1) as f64];
    game.set_bounds(w, h);
    // game.set_message(format!("x:{:?} | y: {:?}", x_bounds, y_bounds));

    let canvas = Canvas::default()
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .block(Block::default().title(TITLE).borders(Borders::ALL))
        .marker(symbols::Marker::Block)
        .paint(|ctx| {
            for chunk in &game.loaded_chunks {
                chunk.draw(ctx);
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
