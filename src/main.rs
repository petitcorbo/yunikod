use crossterm::{execute, terminal};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use game::ui;

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
  
    let file = std::fs::File::open("src/ui/config/locales/langs.json")
    .expect("File should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
    .expect("File should be proper JSON");
    let j_key = json.get("settings.language.short").expect("Key not found");
    let ob = j_key.as_object().unwrap();
    let mut lang = current_locale::current_locale().unwrap();
    for i in 0..ob.len(){
      let ob_cmp = ob.get(&i.to_string()).unwrap().as_str().unwrap();
      if !(lang==ob_cmp.to_string()){
        lang= "en-US".to_string();
      }
    }
    // run game \\
    let status = ui::main_menu::run(&mut terminal, &mut lang);

    // restore terminal \\
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(error) = status { println!("{error}"); }

    Ok(())
}