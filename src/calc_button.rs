// this part will be in charge of rendering the button on instanciation, will output the selected name when pressed, must be TUI button ratatui libraries
use ratatui::{
    layout::Alignment,
    style::Style,
    widgets::{Block, Padding, Paragraph},
};

pub struct CalcButton {
    label: String,
    style: Style,
    pub area: ratatui::layout::Rect,
}

impl CalcButton {
    pub fn new(label: &str, style: Style, area: ratatui::layout::Rect) -> Self {
        Self {
            label: label.to_string(),
            style,
            area,
        }
    }
    pub fn contains(&self, x: u16, y: u16) -> bool {
        self.area.contains((x, y).into())
    }
    pub fn widget(&self) -> Paragraph<'_> {
        // el area ya viene recortada por el margen, aqui solo se centra el texto
        let top = self.area.height.saturating_sub(1) / 2;

        Paragraph::new(self.label.as_str())
            .block(Block::default().padding(Padding::top(top)))
            .style(self.style)
            .alignment(Alignment::Center)
    }
    pub fn on_press(&self) -> &str {
        &self.label
    }
}
