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
use rust_i18n::t;
rust_i18n::i18n!("locales");
use crate::ui::main_menu;

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

pub fn run<B: Backend>(terminal: &mut Terminal<B>, lang: &mut String) -> io::Result<()> {
    let mut list_idx = 0;
    loop {
        let color = match list_idx {
            0 => Color::Blue,
            1 => Color::Blue,
            2 => Color::Blue,
            3 => Color::Blue,
            4 => Color::Red,
            _ => Color::Blue
        };
        // draw \\
        terminal.draw(|frame| draw(frame, list_idx, color, lang.to_string()))?;

        // input handler \\
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Up => {
                    if list_idx > 0 {list_idx -= 1};
                },
                KeyCode::Down => {
                    if list_idx < 4 {list_idx += 1};
                },
                KeyCode::Enter => {
                    match list_idx {
                        0 => {terminal.clear().expect("Error on change LANGUAGE->MENU"); main_menu::run(terminal, &mut "en".to_string()).expect("Error on change language");},
                        1 => {terminal.clear().expect("Error on change LANGUAGE->MENU"); main_menu::run(terminal, &mut "pt-BR".to_string()).expect("Error on change language");},
                        2 => {terminal.clear().expect("Error on change LANGUAGE->MENU"); main_menu::run(terminal, &mut "ru-RU".to_string()).expect("Error on change language");},
                        3 => {terminal.clear().expect("Error on change LANGUAGE->MENU"); main_menu::run(terminal, &mut "ja-JP".to_string()).expect("Error on change language");},
                        4 => return Ok(()),
                        _ => {}
                    };
                },
                _ => {}
            }
        }

    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, list_idx: usize, color: Color, lang: String) {
    rust_i18n::set_locale(&lang); // set language
    let mut vchunks = Layout::default()
        .constraints([Constraint::Length(7), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),Constraint::Length(3),Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size());
    
    let mut blocks = Vec::new();
    for i in 1..=5 {
        vchunks[i].width = 40;
        vchunks[i].x = frame.size().width/2-20;
        if list_idx == i-1 {
            blocks.push(Block::default().borders(Borders::ALL).style(Style::default().fg(color)))
        } else {
            blocks.push(Block::default().borders(Borders::ALL))
        }
    }
    
    let para_title = Paragraph::new(build_title())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(para_title, vchunks[0]);
    
    let para_l1 = Paragraph::new(Span::styled(t!("settings.language.full.0"), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        .block(blocks[0].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_l1, vchunks[1]);

    let para_l2 = Paragraph::new(Span::styled(t!("settings.language.full.1"), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        .block(blocks[1].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_l2, vchunks[2]);

    let para_l3 = Paragraph::new(Span::styled(t!("settings.language.full.2"), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        .block(blocks[2].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_l3, vchunks[3]);

    let para_l4 = Paragraph::new(Span::styled(t!("settings.language.full.3"), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        .block(blocks[3].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_l4, vchunks[4]);

    let para_exit = Paragraph::new(Span::styled(t!("main.opt.back"), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)))
        .block(blocks[4].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_exit, vchunks[5]);
}