use crossterm::{execute, terminal};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use game::{game::{Game, run}, entities::player::Player, items::{ItemKind, flamethrower::FlameThrower}};

fn main() -> Result<(), io::Error> {
    // setup terminal \\
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run game \\
    let game = Game::new();
    let mut player = Player::new(0.0, 0.0);
    player.pick_up(ItemKind::FT(FlameThrower));
    let status = run(&mut terminal, game, player);

    // restore terminal \\
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(error) = status {
        println!("{error}");
    }

    Ok(())
}
