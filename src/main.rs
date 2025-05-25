mod ascii_graphics;
mod dice_generation;
mod menu_choice;
mod trying_ratatui;

use menu_choice::MenuChoice;
use std::io::Write;
use std::process::exit;
use std::str::FromStr;

// TODO: add D&D mode (exact set of dice, nat 0 / nat 20, etc.)

use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let menu_items = vec!["Random Number", "Flip Coin", "Roll Dice", "Exit"];
    let menu: Vec<ListItem> = menu_items
        .iter()
        .map(|&item| ListItem::new(Line::from(vec![Span::raw(item)])))
        .collect();

    loop {
        terminal.draw(|f| {
            let area = f.area();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [Constraint::Percentage(30), Constraint::Percentage(70)].as_ref(),
                )
                .split(area);

            let menu_list = List::new(menu.clone())
                .block(Block::default().title(" Menu ").borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            f.render_widget(menu_list, chunks[0]);

            let main_panel = Paragraph::new("Welcome to RNG Terminal!")
                .block(Block::default().title(" Output ").borders(Borders::ALL));

            f.render_widget(main_panel, chunks[1]);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn print_and_flush(str_to_print: &str) {
    print!("{}", str_to_print);
    std::io::stdout().flush().unwrap(); // NOTE: could handle the error case, maybe
}

fn print_flush_and_clear(str_to_print: &str, mut string_to_clear: String) -> String {
    print_and_flush(str_to_print);
    string_to_clear.clear();
    return string_to_clear;
}

fn get_valid_int<I: FromStr + PartialOrd>(min: I, max: I) -> I
where
    <I as FromStr>::Err: std::fmt::Display,
{
    let mut input_line = String::new();
    let mut user_int;

    'until_valid: loop {
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read the input line");
        let check_int = input_line.trim().parse::<I>();
        match check_int {
            Ok(correct_int) => user_int = correct_int,
            Err(e) => {
                input_line = print_flush_and_clear(&format!("{e}. Try again: "), input_line);
                continue 'until_valid;
            }
        }

        // up to here, the value is guaranteed to be an integer (type I)
        if user_int < min {
            input_line = print_flush_and_clear("The value is too small. Try again: ", input_line);
            continue 'until_valid;
        } else if user_int > max {
            input_line = print_flush_and_clear("The value is too large. Try again: ", input_line);
            continue 'until_valid;
        } else {
            return user_int;
        }
    }
}
