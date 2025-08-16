use color_eyre::eyre::{Ok, Result};
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::ToSpan;
use ratatui::widgets::{HighlightSpacing, Padding};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    input_value: String,
    state: State,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

enum FormAction {
    None,
    Submit,
    Escape,
}

#[derive(Default, Debug)]
enum State {
    #[default]
    List,
    AddNew,
    Update,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    println!("Cada print que se hace en terminal, es costosa");

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
            match app_state.state {
                State::AddNew => {
                    // Se encarga del estado Add New
                    match handle_input_form(key, app_state) {
                        FormAction::Submit => {
                            app_state.state = State::List;
                            app_state.items.push(TodoItem {
                                is_done: false,
                                description: app_state.input_value.clone(),
                            });
                            app_state.input_value.clear();
                        }
                        FormAction::Escape => {
                            app_state.state = State::List;
                            app_state.input_value.clear()
                        }
                        FormAction::None => {}
                    }
                }
                State::Update => {
                    match handle_input_form(key, app_state) {
                        FormAction::Submit => {
                            // Se obtiene el item para actualizarlo.
                            if let Some(index) = app_state.list_state.selected() {
                                if let Some(item) = app_state.items.get_mut(index) {
                                    item.description = app_state.input_value.clone();
                                }
                            }

                            app_state.state = State::List;
                            app_state.input_value.clear();
                        }
                        FormAction::Escape => {
                            app_state.state = State::List;
                            app_state.input_value.clear()
                        }
                        FormAction::None => {}
                    }
                }
                State::List => {
                    // Si es True entonces el programa termina.
                    if handle_key_list_events(key, app_state) {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

// Renderiza de acuerdo al estado actual del programa.
fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    match app_state.state {
        State::AddNew => {
            render_input_form(frame, app_state, border_area);
        }
        State::Update => {
            render_input_form(frame, app_state, border_area);
        }
        State::List => {
            render_list(frame, app_state, border_area);
        }
    }
}

// Renderiza el formulario de entrada de datos.
fn render_input_form(frame: &mut Frame, app_state: &mut AppState, border_area: Rect) {
    let title = match app_state.state {
        State::AddNew => "Ingresa tu tarea:",
        State::Update => "Edita tu tarea:",
        _ => unreachable!(),
    };

    Paragraph::new(app_state.input_value.as_str())
        .block(
            Block::bordered()
                .title(title.to_span().into_centered_line())
                .fg(Color::Green)
                .padding(Padding::ZERO)
                .border_type(BorderType::Rounded),
        )
        .render(border_area, frame.buffer_mut());
}

// Renderiza la lista de tareas.
fn render_list(frame: &mut Frame, app_state: &mut AppState, border_area: Rect) {
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    // Título y bordes
    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .title(
            "App Tasks Perro Malo"
                .to_span()
                .style(Style::default().fg(Color::Yellow))
                .into_centered_line(),
        )
        .render(border_area, frame.buffer_mut());

    if app_state.items.len() > 0 {
        let list = List::new(app_state.items.iter().map(|x| {
            let value = if x.is_done {
                x.description.to_span().crossed_out()
            } else {
                x.description.to_span()
            };
            ListItem::from(value)
        }))
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(Style::default().fg(Color::Cyan));

        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    } else {
        Paragraph::new("No hay tareas.")
            .centered()
            .fg(Color::Red)
            .render(inner_area, frame.buffer_mut());
    }
}

// Maneja los eventos del estado Agregar nuevo.
fn handle_input_form(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        KeyCode::Enter => {
            return FormAction::Submit;
        }
        KeyCode::Esc => {
            return FormAction::Escape;
        }
        _ => {}
    }
    FormAction::None
}

// Maneja los eventos de la lista.
fn handle_key_list_events(key_event: KeyEvent, app_state: &mut AppState) -> bool {
    match key_event.code {
        KeyCode::Enter => {
            if let Some(index) = app_state.list_state.selected() {
                if let Some(item) = app_state.items.get_mut(index) {
                    item.is_done = !item.is_done;
                }
            }
        }
        KeyCode::Esc => return true,
        KeyCode::Char('q') => return true,
        KeyCode::Up => app_state.list_state.select_previous(),
        KeyCode::Down => {
            app_state.list_state.select_next();
        }
        // Eventos del teclado que realizan acciones sobre la lista.
        KeyCode::Char(char) => match char {
            'A' => {
                // Agregar nuevo
                app_state.state = State::AddNew;
            }
            'E' => {
                // Editar
                if let Some(index) = app_state.list_state.selected() {
                    if let Some(item) = app_state.items.get(index) {
                        app_state.input_value = item.description.clone();
                        app_state.state = State::Update;
                    }
                }
            }
            'D' => {
                // Borrar
                // Esta es una construcción muy común en Rust para manejar valores
                // `Option`. Se puede leer como: "Si el resultado de list_state.selected() es, extrae el valor que está adentro,
                // guárdalo en una nueva variable llamada, y ejecuta el código entre las llaves `{}`".
                if let Some(index) = app_state.list_state.selected() {
                    app_state.items.remove(index);
                }
            }
            'k' => {
                // Mover hacia arriba
                app_state.list_state.select_previous();
            }
            'j' => {
                // Mover hacia abajo
                app_state.list_state.select_next();
            }
            _ => {}
        },
        _ => {}
    }
    false
}