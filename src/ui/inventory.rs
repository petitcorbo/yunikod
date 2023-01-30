use crossterm::event::{self, Event, KeyCode};
use perlin2d::PerlinNoise2D;
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
    player::Player, Direction
}, blocks::BlockKind, chunk::{Chunk, CHUNK_SIZE}, game::Game};

pub fn run<B: Backend>(terminal: &mut Terminal<B>, game: &mut Game, mut player: &mut Player) -> io::Result<()> {
    let mut list_idx = 0;
    loop {
        // draw \\
        terminal.draw(|frame| draw(frame, &game, &mut player, list_idx))?;

        // input handler \\
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Up => {
                    if list_idx > player.inventory().len() {list_idx -= 1};
                },
                KeyCode::Down => {
                    if list_idx < player.inventory().len() {list_idx += 1};
                },
                _ => {}
            }
        }

    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, game: &Game, player: &mut Player, list_idx: usize) {
    let vchunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(3), Constraint::Length(3)])
        .split(frame.size());

    let hchunks0 = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(tui::layout::Direction::Horizontal)
        .split(vchunks[0]);

    let hchunks1 = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(tui::layout::Direction::Horizontal)
        .split(vchunks[1]);

    let gauge_foodbar = Gauge::default()
        .block(Block::default().title("[Food]").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::LightYellow))
        .ratio(player.life_ratio());
    frame.render_widget(gauge_foodbar, hchunks0[0]);

    let gauge_lifebar = Gauge::default()
        .block(Block::default().title("[Life]").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(player.life_ratio());
    frame.render_widget(gauge_lifebar, hchunks0[1]);

    let para_using = Paragraph::new(format!("using:"))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(para_using, hchunks1[0]);

    let para_equiped = Paragraph::new(format!("equiped:"))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(para_equiped, hchunks1[1]);

    let mut list_state = ListState::default();
    list_state.select(Some(list_idx));
    let list = List::new(player.inventory().to_extended_item_list())
        .block(Block::default().borders(Borders::ALL))
        .highlight_symbol(">");
    frame.render_stateful_widget(list, vchunks[2], &mut list_state);

    let para_action = Paragraph::new("action")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(para_action, vchunks[3]);
}
