mod ascii_graphics;
mod dice_generation;
mod gui;
mod helper_functions;
mod menu_choice;

use std::io::Write;
use std::str::FromStr;

// TODO: add D&D mode (exact set of dice, nat 0 / nat 20, etc.)

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use gui::app::App;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    return app_result;
}
