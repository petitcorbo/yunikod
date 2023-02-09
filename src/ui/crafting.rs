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
    player::Player, Direction, Recipe
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
                KeyCode::Enter => {
                    let msg = player.inventory().craft(&Recipe::recipes()[list_idx]);
                    game.set_message(msg);
                },
                KeyCode::Up => {
                    if list_idx > 0 {list_idx -= 1};
                },
                KeyCode::Down => {
                    if list_idx < Recipe::recipes().len() - 1 {list_idx += 1};
                },
                _ => {}
            }
        }

    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, game: &Game, player: &mut Player, list_idx: usize) {
    let vchunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(3), Constraint::Length(6), Constraint::Length(3)])
        .split(frame.size());

    let hchunks0 = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(tui::layout::Direction::Horizontal)
        .split(vchunks[0]);

    let para_help = Paragraph::new("help")
        .block(Block::default().title("[Food]").borders(Borders::ALL));
    frame.render_widget(para_help, hchunks0[0]);

    let gauge_lifebar = Gauge::default()
        .block(Block::default().title("[Life]").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(player.life_ratio());
    frame.render_widget(gauge_lifebar, hchunks0[1]);

    let mut list_state = ListState::default();
    list_state.select(Some(list_idx));
    let list = List::new(Recipe::item_list(player.inventory()))
        .block(Block::default().borders(Borders::ALL))
        .highlight_symbol(">");
    frame.render_stateful_widget(list, vchunks[1], &mut list_state);

    let selected_recipe = &Recipe::recipes()[list_idx];
    let para_needs = Paragraph::new(selected_recipe.information(player.inventory()))
        .block(Block::default().borders(Borders::ALL).title(selected_recipe.name()));
    frame.render_widget(para_needs, vchunks[2]);

    let para_action = Paragraph::new(game.message())
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(para_action, vchunks[3]);
}
