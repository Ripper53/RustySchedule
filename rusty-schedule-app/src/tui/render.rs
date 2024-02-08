use ratatui::{layout::{Margin, Rect}, style::{Color, Style}, widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState}, Frame};

use super::UserInterfaceState;

pub fn render(frame: &mut Frame, state: &UserInterfaceState) {
    let size = frame.size();
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    for (index, task, selected) in state.tasks().iter().enumerate().map(|(index, task)| (index, task, state.selected_index() == index)) {
        let task_input_field = Paragraph::new(task.content.value())
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_style(if selected { Color::Cyan } else { Color::LightBlue })
                    .title(task.title.value()),
            )
            .scroll((index as u16, 0));
        frame.render_widget(task_input_field, Rect::new(size.x, size.y, size.width - 1, size.height));
    }
    let mut scrollbar_state = ScrollbarState::new(state.tasks().len()).position(state.focused_task_index);
    frame.render_stateful_widget(scrollbar, size, &mut scrollbar_state);
}
