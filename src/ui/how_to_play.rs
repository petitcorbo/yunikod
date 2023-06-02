use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout,  Rect},
    style::{Color, Style, Modifier},
    layout::Alignment,
    widgets::{Block, Borders, Paragraph, ListItem, List},text::{Spans, Span, Text},
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

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let list_idx = 0;
    loop {
        let color = match list_idx {
            0 => Color::Red,
            _ => Color::Blue
        };

        terminal.draw(|frame| draw(frame, list_idx, color))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Enter =>  return Ok(()),
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
    .expect("File should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
    .expect("File should be a valid JSON");
    let j_key = json.get("settings.language.short").expect("Key not found");
    let ob = j_key.as_object().unwrap();
    let mut lang = current_locale::current_locale().unwrap();
    for i in 0..ob.len(){
      let ob_cmp = ob.get(&i.to_string()).unwrap().as_str().unwrap();
      if !(lang==ob_cmp.to_string()){
        lang= "en-US".to_string();
      }
    }
    let para_title = Paragraph::new(build_title())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(para_title, vchunks[0]);

    let items = [ListItem::new(Spans(vec![Span::raw(t!("htp.keys",lang)),
                                          Span::raw(":")
                                         ])),
                 ListItem::new(" "),
                 ListItem::new(t!("htp.keys.arrows",lang)), 
                 ListItem::new(t!("htp.keys.inventory",lang)), 
                 ListItem::new(t!("htp.keys.crafting",lang)),
                 ListItem::new(t!("htp.keys.map",lang)), 
                 ListItem::new(t!("htp.keys.action",lang)),
                 ListItem::new(" "),
                 ListItem::new("Sprites: "),
                 ListItem::new(" "),
                 ListItem::new(Spans(vec![Span::styled(" ",Style::default().bg(Color::Rgb(54, 181, 201))),
                                         Span::raw(" = "),
                                         Span::raw(t!("htp.sprites.deepwater",lang))])), 
                 ListItem::new(" "),
                 ListItem::new(Spans(vec![Span::styled(" ",Style::default().bg(Color::Rgb(54, 201, 148))),
                                         Span::raw(" = "),
                                         Span::raw(t!("htp.sprites.water",lang))])), 
                 ListItem::new(" "),
                 ListItem::new(Spans(vec![Span::styled(" ",Style::default().bg(Color::Rgb(70, 201, 54))),
                                         Span::raw(" = "),
                                         Span::raw(t!("htp.sprites.grass",lang))])), 
                 ListItem::new(" "),
                 ListItem::new(Spans(vec![Span::styled(" ",Style::default().bg(Color::Rgb(84, 106, 78))),
                                         Span::raw(" = "),
                                         Span::raw(t!("htp.sprites.stone",lang))])), 
                ];
    let par = List::new(items)
    .block(Block::default().title(t!("htp.title",lang)).borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");
    frame.render_widget(par,Rect::new(frame.size().width/2-48, 7, 95, 20));
    let para_exit = Paragraph::new(Span::styled(t!("opt.back",lang), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)))
        .block(blocks[0].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_exit, Rect::new(frame.size().width/2-20, 27, 40, 3));
}
