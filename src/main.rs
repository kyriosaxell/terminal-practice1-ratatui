use color_eyre::eyre::{Ok, Result};
use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

//
fn main() -> Result<()> {
    let mut state = AppState::default();
    println!("Cada print que se hace en terminal, es costosa");

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Estoy cansado jefe"),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Estoy cansado jefe"),
    });

    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

/// Runs the main application loop, handling rendering and input processing.
fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char(char) => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(2)])
        .margin(1)
        .areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .title("Main App Perro Malo")
        .render(border_area, frame.buffer_mut());

    List::new(
        app_state
            .items
            .iter()
            .map(|x| ListItem::from(x.description.clone())),
    )
    .render(inner_area, frame.buffer_mut());
}