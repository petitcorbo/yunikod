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
use crate::ui::config;
use rust_i18n::t;
rust_i18n::i18n!("locales");

fn build_title<'a>(color: Color) -> Text<'a> {
    let style = Style::default().fg(color);

    Text::from(vec![
        Spans(vec![
            Span::raw(" _______  _______  _______  _______  _______  "),
            Span::styled("_______  _______  _______ ", style)
        ]),
        Spans(vec![
            Span::raw("|     __||    ___||_     _||_     _||_     _|"),
            Span::styled("|    |  ||     __||     __|", style)
        ]),
        Spans(vec![
            Span::raw("|__     ||    ___|  |   |    |   |   _|   |_ "),
            Span::styled("|       ||    |  ||__     |", style)
        ]),
        Spans(vec![
            Span::raw("|_______||_______|  |___|    |___|  |_______|"),
            Span::styled("|__|____||_______||_______|", style)
        ]),
    ])
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, lang: &mut String) -> io::Result<()> {
    let mut list_idx = 0;
    loop {
        let color = match list_idx {
            0 => Color::Blue,
            1 => Color::Red,
            _ => Color::Blue
        };

        terminal.draw(|frame| draw(frame, list_idx, color, lang.to_string()))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Up => {
                    if list_idx > 0 {list_idx -= 1};
                },
                KeyCode::Down => {
                    if list_idx < 1 {list_idx += 1};
                },
                KeyCode::Enter => {
                    match list_idx {
                        0 => {terminal.clear().expect("Error on change SETTINGS->LANGUAGE"); config::language::run(terminal, lang)}?,
                        1 => return Ok(()),
                        _ => {}
                    }
                },
                _ => {}
            }
        }

    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, list_idx: usize, color: Color, lang: String) {
    rust_i18n::set_locale(&lang); // set language
    let mut vchunks = Layout::default()
        .constraints([Constraint::Length(7), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size());
    
    let mut blocks = Vec::new();
    for i in 1..=2 {
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
    
    let para_language = Paragraph::new(Span::styled(t!("settings.language.opt"), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        .block(blocks[0].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_language, vchunks[1]);

    let para_back = Paragraph::new(Span::styled(t!("main.opt.back"), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)))
        .block(blocks[1].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_back, vchunks[2]);
}
