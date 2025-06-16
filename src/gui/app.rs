use crate::{dice_generation, menu_choice::get_all_menu_strings};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal, Frame,
};
use std::io;

#[derive(Default, Debug)]
pub enum AppState {
    #[default]
    Menu,
    InputCustomDice {
        buffer: String,
    },
}

#[derive(Debug, Default)]
pub struct App {
    menu_items: Vec<String>,
    selected: usize,
    state: AppState,
    output: String,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            menu_items: get_all_menu_strings(),
            selected: 0,
            state: AppState::Menu,
            output: String::from("Welcome to RNG Terminal!"),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_stateful_widget(self, frame.area(), &mut ListState::default());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match &mut self.state {
            AppState::Menu => match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Down | KeyCode::Char('j') => self.next_option(),
                KeyCode::Up | KeyCode::Char('k') => self.prev_option(),
                KeyCode::Enter => match self.selected {
                    0 => self.output = format!("Coin: {}", dice_generation::flip_coin()),
                    1 => self.output = format!("Random: {}", dice_generation::standard_dice()),
                    2 => {
                        self.state = AppState::InputCustomDice {
                            buffer: String::new(),
                        };
                        self.output.clear();
                    }
                    3 => self.exit = true,
                    _ => {}
                },
                _ => {}
            },
            AppState::InputCustomDice { buffer } => match key_event.code {
                KeyCode::Char(c) if c.is_ascii_digit() => buffer.push(c),
                KeyCode::Backspace => {
                    buffer.pop();
                }
                KeyCode::Enter => {
                    if let Ok(max_val) = buffer.parse::<u8>() {
                        self.output = format!(
                            "Custom Dice ({}): {}",
                            max_val,
                            dice_generation::generate_dice_result(1, max_val as i8)
                        );
                    } else {
                        self.output = "Invalid number".into();
                    }
                    self.state = AppState::Menu;
                }
                KeyCode::Esc => {
                    self.state = AppState::Menu;
                    self.output = "Cancelled".into();
                }
                _ => {}
            },
        }
    }

    fn next_option(&mut self) {
        if self.selected != self.menu_items.len() - 1 {
            self.selected += 1
        } else {
        }
    }

    fn prev_option(&mut self) {
        if self.selected != 0 {
            self.selected -= 1;
        } else {
        }
    }
}

impl StatefulWidget for &App {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let menu_title = Line::from("Menu");
        let instructions = Line::from(vec![
            " Up ".into(),
            "<ArrowUp>/<k>".blue().bold(),
            " Down ".into(),
            "<ArrowDown>/<j>".blue().bold(),
            " Quit ".into(),
            "<q>".bold().blue(),
        ]);
        let menu_block = Block::bordered()
            .title(menu_title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        // Prepare list items
        let items: Vec<ListItem> = self
            .menu_items
            .iter()
            .map(|item| ListItem::new(Line::from(Span::raw(item))))
            .collect();
        // Render menu with highlight
        state.select(Some(self.selected));
        StatefulWidget::render(
            List::new(items)
                .block(menu_block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("=> "),
            chunks[0],
            buf,
            state,
        );

        // Determine main content text
        let main_text = match &self.state {
            AppState::Menu => self.output.clone(),
            AppState::InputCustomDice { buffer } => format!("Enter number of sides: {}", &buffer),
        };
        // Render output pane
        Paragraph::new(main_text)
            .block(
                Block::default()
                    .title("RNG Terminal by KS")
                    .borders(Borders::ALL),
            )
            .render(chunks[1], buf);
    }
}
