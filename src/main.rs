// ---------- mods --------------
mod app;
mod calc_button;
mod expresion_parser;

use app::App;
use calc_button::CalcButton;

// ---------- uses --------------
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, stdout};

use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

const BUTTON_ROWS: usize = 6;
const BUTTON_COLS: usize = 4;
const LOGO_ROWS: usize = 3;
const LOGO_WIDTH: u16 = 41;

const LABELS: [[&str; BUTTON_COLS]; BUTTON_ROWS] = [
    ["AC", "⌫", "%", "√"],
    ["7", "8", "9", "/"],
    ["4", "5", "6", "*"],
    ["1", "2", "3", "-"],
    ["0", ".", "(", ")"],
    ["±", "^", "=", "+"],
];

const LOGO: [&str; LOGO_ROWS] = [
    "█████ █   █ █     █████ █████ █   █ █████",
    "█     █████ █     █       █   █   █   █  ",
    "█████ █   █ █████ █████   █   █████ █████",
];

// con menos de 21 lineas el logo se oculta para no apretar los botones
const MIN_HEIGHT_FOR_LOGO: u16 = 21;

fn hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    Color::Rgb(r, g, b)
}

fn button_style(
    label: &str,
    button_color: Color,
    operation_color: Color,
    function_color: Color,
) -> Style {
    match label {
        "+" | "-" | "*" | "/" | "^" | "%" | "=" => Style::default().bg(operation_color),

        "√" | "±" | "(" | ")" | "AC" | "⌫" => Style::default().bg(function_color),

        _ => Style::default().bg(button_color),
    }
}

/// Devuelve [logo, resultado, input, botones]. El logo se oculta si no entra.
fn layout(area: Rect) -> [Rect; 4] {
    let logo_height = if shows_logo(area) {
        LOGO_ROWS as u16
    } else {
        0
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(logo_height),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(area);

    [chunks[0], chunks[1], chunks[2], chunks[3]]
}

fn shows_logo(area: Rect) -> bool {
    area.height >= MIN_HEIGHT_FOR_LOGO && area.width >= LOGO_WIDTH
}

fn create_buttons(
    area: Rect,
    button_color: Color,
    operation_color: Color,
    function_color: Color,
) -> Vec<CalcButton> {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1); BUTTON_ROWS])
        .split(area);

    // con terminales chicos se achica el margen para que los botones sigan siendo clickeables
    let vertical_margin = if area.height / BUTTON_ROWS as u16 >= 3 {
        1
    } else {
        0
    };

    let mut buttons = Vec::new();

    for (row_index, row) in rows.iter().enumerate() {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1); BUTTON_COLS])
            .split(*row);

        for (col_index, column) in columns.iter().enumerate() {
            let label = LABELS[row_index][col_index];

            let style = button_style(label, button_color, operation_color, function_color);
            let area = column.inner(Margin {
                vertical: vertical_margin,
                horizontal: 1,
            });

            buttons.push(CalcButton::new(label, style, area));
        }
    }

    buttons
}

fn press(app: &mut App, label: &str) {
    match label {
        "=" => app.evaluate(),
        "AC" => app.clear(),
        "⌫" => app.backspace(),
        "±" => app.toggle_sign(),
        _ => {
            for c in label.chars() {
                app.add_input(c);
            }
        }
    }
}

fn draw_frame(
    frame: &mut Frame,
    color: Color,
    button_color: Color,
    operation_color: Color,
    function_color: Color,
    logo_color: Color,
    app: &App,
) {
    let area = frame.area();
    let layout = layout(area);
    let background = Block::default().style(Style::default().bg(color));

    frame.render_widget(background, area);

    // ------------------------------
    // --------Logo ----------------
    //-------------------------------
    if shows_logo(area) {
        let logo = Paragraph::new(LOGO.join("\n"))
            .style(Style::default().fg(logo_color))
            .alignment(Alignment::Center);

        frame.render_widget(logo, layout[0]);
    }

    let result_style = if app.error {
        Style::default().fg(hex_color("#ff5f56"))
    } else {
        Style::default()
    };

    let result = Paragraph::new(app.result.as_str())
        .block(Block::default().borders(Borders::ALL))
        .style(result_style)
        .alignment(Alignment::Right);

    frame.render_widget(result, layout[1]);

    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Right);

    frame.render_widget(input, layout[2]);

    // ------------------------------
    // --------Buttons --------------
    //-------------------------------
    let buttons = create_buttons(layout[3], button_color, operation_color, function_color);
    for button in buttons {
        frame.render_widget(button.widget(), button.area);
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    loop {
        terminal.draw(|frame| {
            draw_frame(
                frame,
                hex_color("#02010a"),
                hex_color("#11192a"),
                hex_color("#4c5950"),
                hex_color("#2c3550"),
                hex_color("#9ece6a"),
                &app,
            );
        })?;

        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => break,

                KeyCode::Char('c') | KeyCode::Char('C') => app.clear(),

                KeyCode::Char('=') | KeyCode::Enter => app.evaluate(),

                KeyCode::Char(c) => app.add_input(c),

                KeyCode::Backspace => app.backspace(),

                _ => {}
            },

            Event::Mouse(mouse) if mouse.kind == MouseEventKind::Down(MouseButton::Left) => {
                let size = terminal.size()?;

                let terminal_area = Rect::new(0, 0, size.width, size.height);

                let buttons = create_buttons(
                    layout(terminal_area)[3],
                    hex_color("#11192a"),
                    hex_color("#4c5950"),
                    hex_color("#2c3550"),
                );

                for button in buttons {
                    if button.contains(mouse.column, mouse.row) {
                        press(&mut app, button.on_press());
                    }
                }
            }

            _ => {}
        }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, buffer::Buffer};

    fn render_buffer(width: u16, height: u16, app: &App) -> Buffer {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("terminal valido");
        let frame = terminal
            .draw(|frame| {
                draw_frame(
                    frame,
                    hex_color("#02010a"),
                    hex_color("#11192a"),
                    hex_color("#4c5950"),
                    hex_color("#2c3550"),
                    hex_color("#9ece6a"),
                    app,
                )
            })
            .expect("draw valido");

        frame.buffer.clone()
    }

    fn render(width: u16, height: u16, app: &App) -> String {
        let buffer = render_buffer(width, height, app);
        buffer.content.iter().map(|cell| cell.symbol()).collect()
    }

    fn line_at(buffer: &Buffer, y: u16) -> String {
        (0..buffer.area.width)
            .map(|x| buffer[(x, y)].symbol())
            .collect()
    }

    #[test]
    fn renders_every_button() {
        let app = App::new();
        let screen = render(80, 24, &app);

        for row in LABELS {
            for label in row {
                assert!(screen.contains(label), "falta el boton {label}");
            }
        }
    }

    #[test]
    fn renders_input_and_result() {
        let mut app = App::new();
        for c in "2+3*4".chars() {
            app.add_input(c);
        }
        assert!(render(80, 24, &app).contains("2+3*4"));

        app.evaluate();
        assert!(render(80, 24, &app).contains("14"));
    }

    #[test]
    fn renders_the_logo_when_there_is_room() {
        let app = App::new();
        let screen = render(80, 24, &app);

        for row in LOGO {
            assert!(screen.contains(row), "falta una fila del logo");
        }
    }

    #[test]
    fn hides_the_logo_when_it_does_not_fit() {
        let app = App::new();
        // sin alto suficiente y sin ancho suficiente
        assert!(!render(80, 16, &app).contains("█████"));
        assert!(!render(30, 24, &app).contains("█████"));
    }

    #[test]
    fn buttons_are_clickable_where_they_are_drawn() {
        let app = App::new();
        let buffer = render_buffer(80, 24, &app);
        let area = layout(buffer.area)[3];

        let buttons = create_buttons(
            area,
            hex_color("#11192a"),
            hex_color("#4c5950"),
            hex_color("#2c3550"),
        );
        assert_eq!(buttons.len(), BUTTON_ROWS * BUTTON_COLS);

        for button in buttons {
            let label = button.on_press();

            // el texto puede caer en cualquiera de las filas del boton segun su alto
            let spot = (button.area.y..button.area.bottom())
                .find_map(|y| line_at(&buffer, y).find(label).map(|x| (x as u16, y)))
                .unwrap_or_else(|| panic!("no se dibuja el boton {label}"));

            assert!(
                button.contains(spot.0, spot.1),
                "el boton {label} se dibuja fuera de su area clickeable"
            );
        }
    }
}
