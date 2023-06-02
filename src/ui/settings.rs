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
use locales::t;

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

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut list_idx = 0;
    loop {
        let color = match list_idx {
            0 => Color::Blue,
            1 => Color::Red,
            _ => Color::Blue
        };

        terminal.draw(|frame| draw(frame, list_idx, color))?;

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
                        0 => config::language::run(terminal)?,
                        1 => return Ok(()),
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
    for i in 1..=2 {
        vchunks[i].width = 40;
        vchunks[i].x = frame.size().width/2-20;
        if list_idx == i-1 {
            blocks.push(Block::default().borders(Borders::ALL).style(Style::default().fg(color)))
        } else {
            blocks.push(Block::default().borders(Borders::ALL))
        }
    }
    let file = std::fs::File::open("src/ui/config/locales/langs.json")
    .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
    .expect("file should be proper JSON");
    let j_key = json.get("settings.language.short").expect("Key not found");
    let ob = j_key.as_object().unwrap();
    let mut lang = current_locale::current_locale().unwrap();
    for i in 0..ob.len(){
      let ob_cmp = ob.get(&i.to_string()).unwrap().as_str().unwrap();
      if !(lang==ob_cmp.to_string()){
        lang= "en-US".to_string();
      }
    }
    let para_title = Paragraph::new(build_title(color))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(para_title, vchunks[0]);
    
    let para_new_game = Paragraph::new(Span::styled(t!("settings.opt.language",lang), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        .block(blocks[0].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_new_game, vchunks[1]);

    let para_exit = Paragraph::new(Span::styled(t!("opt.back",lang), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)))
        .block(blocks[1].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_exit, vchunks[2]);
}
