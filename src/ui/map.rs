use crossterm::event::{self, Event, KeyCode};
use tui::{
    Frame,
    Terminal,
    backend::Backend,
    style::{Style, Color},
    layout::{Layout, Constraint},
    widgets::{Block, Borders, Paragraph, Gauge, canvas::Canvas}, text::{Spans, Span}
};
use std::io;
use crate::{
    entities::player::Player,
    game::Game
};

pub fn run<B: Backend>(terminal: &mut Terminal<B>, game: &mut Game, mut player: &mut Player) -> io::Result<u8> {
    loop {
        // draw \\
        terminal.draw(|frame| draw(frame, &game, &mut player))?;

        // input handler \\
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(0),
                KeyCode::Left => return Ok(2),
                KeyCode::Right => return Ok(4),
                _ => {}
            }
        }

    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, game: &Game, player: &mut Player) {
    let vchunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(3), Constraint::Length(3)])
        .split(frame.size());

    let hchunks0 = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(tui::layout::Direction::Horizontal)
        .split(vchunks[0]);

    let tabs = vec![
        Span::raw("inventory | crafting | "),
        Span::styled("map", Style::default().fg(Color::Green)),
        Span::raw(" | menu"),
    ];
    let para_tabs = Paragraph::new(Spans::from(tabs))
        .block(Block::default().title("[Tab]").borders(Borders::ALL));
    frame.render_widget(para_tabs, hchunks0[0]);

    let gauge_lifebar = Gauge::default()
        .block(Block::default().title("[Life]").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(player.life_ratio());
    frame.render_widget(gauge_lifebar, hchunks0[1]);
    
    let w = (vchunks[1].width - 3) as f64 / 2.0;
    let h = (vchunks[1].height - 3) as f64 / 2.0;
    let canvas = Canvas::default()
        .x_bounds([-w, w])
        .y_bounds([-h, h])
        .block(Block::default().title("map").borders(Borders::ALL))
        .paint(|ctx| {
            for chunk in game.loaded_chunks() {
                ctx.print(chunk.0 as f64, chunk.1 as f64, chunk.average_terrain().span());
            }
            for chunk in game.unused_chunks() {
                ctx.print(chunk.0 as f64, chunk.1 as f64, chunk.average_terrain().span());
            }
            ctx.print((player.x()/16) as f64, (player.y()/16) as f64, Span::styled("+", Style::default().fg(Color::Red)));
        });
    frame.render_widget(canvas, vchunks[1]);

    let para_action = Paragraph::new(game.message())
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(para_action, vchunks[2]);
}
