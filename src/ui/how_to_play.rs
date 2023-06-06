use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout,  Rect},
    style::{Color, Style, Modifier},
    layout::Alignment,
    widgets::{Block, Borders, Paragraph, Table, Row, Cell},text::{Spans, Span, Text},
    Frame, Terminal,
};
use locales::t;

fn build_title<'a>() -> Text<'a> {
  let style = Style::default().fg(Color::Yellow);
    Text::from(vec![
        Spans(vec![
            Span::styled(" __         ___        ",style)
        ]),
        Spans(vec![
            Span::styled("|__|.-----.'  _|.-----.",style),
        ]),
        Spans(vec![
            Span::styled("|  ||     |   _||  _  |",style),
        ]),
        Spans(vec![
            Span::styled("|__||__|__|__|  |_____|",style),
        ]),
    ])
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, lang: &mut String) -> io::Result<()> {
    let list_idx = 0;
    loop {
        let color = match list_idx {
            0 => Color::Red,
            _ => Color::Blue
        };

        terminal.draw(|frame| draw(frame, list_idx, color, lang.to_string()))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Enter =>  return Ok(()),
                _ => {}
            }
        }

    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, list_idx: usize, color: Color, lang: String) {
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
    let para_title = Paragraph::new(build_title())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
  
    frame.render_widget(para_title, vchunks[0]);
  
    let par = Table::new(vec![
      Row::new(vec![
          Cell::from(t!("htp.keys.arrows",lang)),
          Cell::from(Spans::from(vec![
              Span::styled(" ",Style::default().bg(Color::Rgb(54, 181, 201))),
              Span::raw(" = "),
              Span::raw(t!("htp.sprites.deepwater",lang))
          ])),
          Cell::from(Spans::from(vec![
              Span::raw("⇞ = "),
              Span::raw(t!("htp.res.tree",lang)),
          ])),
          Cell::from(Spans::from(vec![
              Span::raw("⣳ = "),
              Span::raw(t!("game.ores.coal",lang)),
          ])),
      ]),
      Row::new(vec![
          Cell::from(t!("htp.keys.inventory",lang)),
          Cell::from(Spans::from(vec![
              Span::styled(" ",Style::default().bg(Color::Rgb(54, 201, 148))),
              Span::raw(" = "),
              Span::raw(t!("htp.sprites.water",lang)),
          ])),
          Cell::from(Spans::from(vec![
              Span::raw("⠣ = "),
              Span::raw(t!("htp.res.stones",lang)),
          ])),
          Cell::from(Spans::from(vec![
              Span::raw("⡵ = "),
              Span::raw(t!("game.ores.iron",lang)),
          ])),
      ]),
      Row::new(vec![
          Cell::from(t!("htp.keys.crafting",lang)),
          Cell::from(Spans::from(vec![
              Span::styled(" ",Style::default().bg(Color::Rgb(70, 201, 54))),
              Span::raw(" = "),
              Span::raw(t!("htp.sprites.grass",lang)),
          ])),
          Cell::from(Spans::from(vec![
              Span::raw("ɻ = "),
              Span::raw(t!("htp.res.sticks",lang)),
          ])),
          Cell::from(Spans::from(vec![
              Span::raw("⡝ = "),
              Span::raw(t!("game.ores.gold",lang)),
          ])),
      ]),
      Row::new(vec![
          Cell::from(t!("htp.keys.map",lang)),
          Cell::from(Spans::from(vec![
              Span::styled(" ",Style::default().bg(Color::Rgb(84, 106, 78))),
              Span::raw(" = "),
              Span::raw(t!("htp.sprites.stone",lang)),
          ])),
          Cell::from(Spans::from(vec![
              Span::raw("⣿ = "),
              Span::raw(t!("htp.res.rock",lang)),
          ])),
          Cell::from("")
      ]),
      Row::new(vec![
          Cell::from(t!("htp.keys.action",lang)),
          Cell::from(Spans::from(vec![
              Span::raw("▲"),
              Span::raw(" = "),
              Span::raw(t!("htp.sprites.player",lang)),
          ])),
          Cell::from(Spans::from(vec![
              Span::raw("; = "),
              Span::raw(t!("htp.res.grasstuft",lang)),
          ])),
          Cell::from("")
      ])
    ]).style(Style::default().fg(Color::White))
    .header(
      Row::new(vec![t!("htp.keys.header",lang), 
                          t!("htp.sprites.header",lang), 
                          t!("htp.res.header",lang),
                          t!("htp.ores.header",lang)]
              )
      .style(Style::default().fg(Color::LightBlue))
      .bottom_margin(1)
    )
    .block(Block::default().title(t!("htp.title",lang)).borders(Borders::ALL))
    .widths(&[Constraint::Length(20), Constraint::Length(20), Constraint::Length(20),Constraint::Length(20)])
    .column_spacing(3);
    vchunks[0].y=frame.size().height/2-20;
    vchunks[0].height=9;
    frame.render_widget(par,vchunks[0]);
    
    let para_exit = Paragraph::new(Span::styled(t!("opt.back",lang), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)))
          .block(blocks[0].clone())
          .alignment(Alignment::Center);
    frame.render_widget(para_exit, Rect::new(frame.size().width/2-20, 16, 40, 3));
}
