use crossterm::event::{self, Event, KeyCode};
use tui::{
    Frame,
    Terminal,
    backend::Backend,
    style::{Style, Color},
    layout::{Layout, Constraint},
    widgets::{Block, Borders, Paragraph, Gauge, List, ListState}, text::{Spans, Span}
};
use std::io;
use crate::{
    entities::player::Player,
    game::Game
};
use locales::t;

pub fn run<B: Backend>(terminal: &mut Terminal<B>, game: &mut Game, mut player: &mut Player, lang: &mut String) -> io::Result<u8> {
    let mut list_idx = 0;
    loop {
        // draw \\
        terminal.draw(|frame| draw(frame, &game, &mut player, list_idx, lang.to_string()))?;

        // input handler \\
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(0),
                KeyCode::Up => {
                    if list_idx > 0 {list_idx -= 1};
                },
                KeyCode::Down => {
                    if list_idx < player.inventory().len() - 1 {list_idx += 1};
                },
                KeyCode::Right => return Ok(2),
                KeyCode::Enter => player.set_using(list_idx),
                _ => {}
            }
        }

    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, _game: &Game, player: &mut Player, list_idx: usize, lang: String) {
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

    let tabs = vec![
        Span::styled(t!("game.ui.tab.inventory",lang), Style::default().fg(Color::Green)),
        Span::raw(format!(" | {} | {} | {}",t!("game.ui.tab.crafting",lang),t!("game.ui.tab.map",lang),t!("game.ui.tab.menu",lang))),
    ];
    let para_tabs = Paragraph::new(Spans::from(tabs))
        .block(Block::default().title(t!("game.ui.tab",lang)).borders(Borders::ALL));
    frame.render_widget(para_tabs, hchunks0[0]);

    let gauge_lifebar = Gauge::default()
        .block(Block::default().title(t!("game.ui.life",lang)).borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(player.life_ratio());
    frame.render_widget(gauge_lifebar, hchunks0[1]);

    let idx = player.using();
    let para_using = Paragraph::new(format!("{} {}",t!("game.ui.using",lang), player.inventory().get(idx).name()))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(para_using, hchunks1[0]);

    let para_equiped = Paragraph::new(format!("{}",t!("game.ui.equiped",lang)))
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
