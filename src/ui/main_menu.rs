use crossterm::event::{self, Event, KeyCode};
use tui::{
    Frame,
    Terminal,
    backend::Backend,
    style::{Style, Color},
    layout::{Layout, Constraint, Alignment},
    widgets::{Block, Borders, Paragraph}, text::{Spans, Span, Text}
};
use std::io;

use crate::{game::{self, Game}, entities::player::Player};

fn build_title<'a>(color: Color) -> Text<'a> {
    let style = Style::default().fg(color);

    Text::from(vec![
        Spans(vec![
            Span::raw(" ___ ___               __       "),
            Span::styled(" __  __           __ ", style)
        ]),
        Spans(vec![
            Span::raw("|   |   |.--.--.-----.|__|______"),
            Span::styled("|  |/  |.-----.--|  |", style)
        ]),
        Spans(vec![
            Span::raw(" \\     / |  |  |     ||  |______"),
            Span::styled("|     < |  _  |  _  |", style)
        ]),
        Spans(vec![
            Span::raw("  |___|  |_____|__|__||__|      "),
            Span::styled("|__|\\__||_____|_____|", style)
        ]),
    ])
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<u8> {
    let mut list_idx = 0;
    loop {
        let color = match list_idx {
            0 => Color::Blue,
            1 => Color::Green,
            2 => Color::Red,
            _ => Color::Blue
        };
        // draw \\
        terminal.draw(|frame| draw(frame, list_idx, color))?;

        // input handler \\
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(0),
                KeyCode::Up => {
                    if list_idx > 0 {list_idx -= 1};
                },
                KeyCode::Down => {
                    if list_idx < 2 {list_idx += 1};
                },
                KeyCode::Enter => {
                    match list_idx {
                        0 => new_game(terminal)?,
                        1 => {},
                        2 => return Ok(0),
                        _ => {}
                    }
                },
                _ => {}
            }
        }

    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, list_idx: usize, color: Color) {
    let mut vchunks = Layout::default()
        .constraints([Constraint::Length(7), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size());
    
    let mut blocks = Vec::new();
    for i in 1..=3 {
        vchunks[i].width = 40;
        vchunks[i].x = frame.size().width/2-20;
        if list_idx == i-1 {
            blocks.push(Block::default().borders(Borders::ALL).style(Style::default().fg(color)))
        } else {
            blocks.push(Block::default().borders(Borders::ALL))
        }
    }

    let para_title = Paragraph::new(build_title(color))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(para_title, vchunks[0]);
    
    let para_new_game = Paragraph::new(Span::styled("NEW GAME", Style::default().fg(Color::Blue)))
        .block(blocks[0].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_new_game, vchunks[1]);
    
    let para_settings = Paragraph::new(Span::styled("SETTINGS", Style::default().fg(Color::Green)))
        .block(blocks[1].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_settings, vchunks[2]);

    let para_exit = Paragraph::new(Span::styled("EXIT", Style::default().fg(Color::Red)))
        .block(blocks[2].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_exit, vchunks[3]);
}

fn new_game<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()>{
    let mut game = Game::new();
    game.update_chunks();
    let mut x = 0.0;
    while game.perlin().get_noise(x, 0.0) < 0.0 {
        x += 1.0
    }
    //let player = Player::new(x as i64, 0);
    let mut player = Player::new(x as i64, 0);
    player.inventory().add(crate::items::ItemKind::Stick(crate::items::stick::Stick::new(20)));
    game::run(terminal, game, player)?;
    Ok(())
}
