use crossterm::{execute, terminal};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use game::{game::{Game, run}, entities::player::Player, items::{ItemKind, axe::Axe}};

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
    loop {
        match game.get_tile(x, 0.0) {
            game::chunk::Terrain::Water => {
                x += 1.0;
            }
            game::chunk::Terrain::DeepWater => {
                x += 1.0;
            }
            _ => { break }
        }
    }
    let mut player = Player::new(x, 0.0);
    player.pick_up(ItemKind::Axe(Axe::new(1)));
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
