use crossterm::event::{self, Event, KeyCode};
use tui::{
    Frame,
    Terminal,
    backend::Backend,
    style::{Style, Color, Modifier},
    layout::{Layout, Constraint, Alignment},
    widgets::{Block, Borders, Paragraph}, text::{Spans, Span, Text}
};
use std::io;
use crate::{game::{self, Game}, entities::player::Player, ui::main_menu};
use locales::t;

fn build_title<'a>() -> Text<'a> {
    let style = Style::default().fg(Color::Red);

    Text::from(vec![
        Spans(vec![
            Span::styled(" _______                            _______                   ", style)
        ]),
        Spans(vec![
            Span::styled("|     __|.---.-.--------.-----.    |       |.--.--.-----.----.", style)
        ]),
        Spans(vec![
            Span::styled("|    |  ||  _  |        |  -__|    |   -   ||  |  |  -__|   _|", style)
        ]),
        Spans(vec![
            Span::styled(r"|_______||___._|__|__|__|_____|    |_______| \___/|_____|__|  ", style)
        ]),
    ])
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, lang: &mut String) -> io::Result<()> {
    let mut list_idx = 0;
    loop {
        let color = match list_idx {
            0 => Color::Green,
            1 => Color::Yellow,
            2 => Color::Blue,
            3 => Color::Red,
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
                    if list_idx < 3 {list_idx += 1};
                },
                KeyCode::Enter => {
                    match list_idx {
                        0 => {terminal.clear().expect("Error on change GAMEOVER->NEW GAME"); new_game(terminal, lang)}?,
                        1 => {terminal.clear().expect("Error on change GAMEOVER->MAIN MENU"); main_menu::run(terminal, lang)}?,
                        _ => {}
                    }
                },
                _ => {}
            }
        }

    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, list_idx: usize, color: Color, lang: String) {
    let mut vchunks = Layout::default()
        .constraints([Constraint::Length(7), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size());
    
    let mut blocks = Vec::new();
    for i in 1..=4 {
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

    let para_new_game = Paragraph::new(Span::styled(t!("main.opt.new_game",lang), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)))
        .block(blocks[0].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_new_game, vchunks[1]);

    let para_main_menu = Paragraph::new(Span::styled(t!("main.opt.main_menu",lang), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
        .block(blocks[1].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_main_menu, vchunks[2]);

}

fn new_game<B: Backend>(terminal: &mut Terminal<B>, lang: &mut String) -> io::Result<()>{
    let mut game = Game::new();
    game.update_chunks();
    let mut x = 0.0;
    while game.perlin().get_noise(x, 0.0) < 0.0 {
        x += 1.0
    }
    let player = Player::new(x as i64, 0);
    game::run(terminal, game, player,lang)?;
    Ok(())
}
