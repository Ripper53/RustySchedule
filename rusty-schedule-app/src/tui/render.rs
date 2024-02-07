use ratatui::{widgets::{Block, Borders, Paragraph}, Frame};

use super::UserInterfaceState;

pub fn render(frame: &mut Frame, state: UserInterfaceState) {
    let input_field = Paragraph::new(state.input.value())
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title("Schedule"),
        );
    frame.render_widget(input_field, frame.size());
}
