use crossterm::{execute, terminal};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use game::{game::{Game, run}, entities::player::Player};

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
    let mut game = Game::new();
    game.update_chunks();
    let mut x = 0.0;
    while game.perlin().get_noise(x, 0.0) < 0.0 {
        x += 1.0
    }
    let player = Player::new(x as i64, 0);
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
