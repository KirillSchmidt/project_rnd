mod dice_generation;
mod gui;
mod helper_functions;
mod menu_choice;
mod teams;
// TODO: add D&D mode (exact set of dice, nat 0 / nat 20, etc.)

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use gui::app::App;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

// fn main() -> io::Result<()> {
fn main() {
    // enable_raw_mode()?;
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;
    //
    // let mut app = App::new();
    // let app_result = app.run(&mut terminal);
    // ratatui::restore();
    // return app_result;
    let mut new_team = teams::Team::new(
        Some("First team!"),
        Some(vec!["Me", "Myself", "I"]),
        Some("I"),
    )
    .expect("Couldn't create a new team");
    println!("{:?}", new_team.get_captain_name());
}
