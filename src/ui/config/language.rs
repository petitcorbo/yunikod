use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Style, Modifier},
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},text::{Spans, Span, Text},
    Frame, Terminal,
};
use locales::t;

fn build_title<'a>() -> Text<'a> {
    Text::from(vec![
        Spans(vec![
            Span::raw(" _____   _______ _______ _______ "),
        ]),
        Spans(vec![
            Span::raw("|     |_|   _   |    |  |     __|"),
        ]),
        Spans(vec![
            Span::raw("|       |       |       |    |  |"),
        ]),
        Spans(vec![
            Span::raw("|_______|___|___|__|____|_______|"),
        ]),
    ])
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut list_idx = 0;
    loop {
        let color = match list_idx {
            0 => Color::Blue,
            1 => Color::Blue,
            2 => Color::Red,
            _ => Color::Blue
        };
        // draw \\
        terminal.draw(|frame| draw(frame, list_idx, color))?;

        // input handler \\
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Up => {
                    if list_idx > 0 {list_idx -= 1};
                },
                KeyCode::Down => {
                    if list_idx < 2 {list_idx += 1};
                },
                KeyCode::Enter => {
                    match list_idx {
                        0 => {},
                        1 => {},
                        2 => return Ok(()),
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
    
    let mut lang = current_locale::current_locale().unwrap();
    if lang=="C"{
        lang = "en-US".to_string();
    }
    let para_title = Paragraph::new(build_title())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(para_title, vchunks[0]);
    
    let para_l1 = Paragraph::new(Span::styled("ENGLISH", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        .block(blocks[0].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_l1, vchunks[1]);

    let para_l2 = Paragraph::new(Span::styled("PORTUGUESE", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        .block(blocks[1].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_l2, vchunks[2]);

    let para_exit = Paragraph::new(Span::styled(t!("opt.back",lang), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)))
        .block(blocks[2].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_exit, vchunks[3]);
}