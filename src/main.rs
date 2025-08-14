use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    widgets::{Paragraph, Widget},
};

fn main() -> Result<()> {
    println!("Cada print que se hace en terminal, es costosa");
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

/// Runs the main application loop, handling rendering and input processing.
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(render)?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

//
fn render(frame: &mut Frame) {
    Paragraph::new("Hola desde la TUI").render(frame.area(), frame.buffer_mut());
}
