use ratatui::{layout::{Constraint, Direction, Layout, Margin, Rect}, style::{Color, Style}, widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState}, Frame};

use super::{TaskFocus, UserInterfaceState};

pub fn render(frame: &mut Frame, state: &UserInterfaceState) {
    let size = frame.size();
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"))
        .style(Color::Cyan);
    let height = size.height;
    for (index, task, selected) in state.tasks().iter().enumerate().map(|(index, task)| (index, task, state.selected_index() == index)) {
        let [task_layout, time_layout] = Layout::new(Direction::Vertical, [
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(Rect::new(size.x, size.y, size.width - 1, height));
        let default_color = if selected { Color::Cyan } else { Color::LightBlue };
        let scroll = (index as u16, 0);
        let focus_type = state.focus_type();
        let task_input_field = Paragraph::new(task.content.value())
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_style(if focus_type == TaskFocus::Content { Color::LightYellow }  else { default_color })
                    .title(task.title.value())
                    .title_style(if focus_type == TaskFocus::Title { Color::LightYellow }  else { default_color }),
            )
            .scroll(scroll);
        frame.render_widget(task_input_field, task_layout);
        let time_input_field = Paragraph::new(task.time.value())
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_style(if focus_type == TaskFocus::Time { Color::LightYellow } else { default_color })
                    .title("Time"),
            )
            .scroll(scroll);
        frame.render_widget(time_input_field, time_layout);
    }
    let mut scrollbar_state = ScrollbarState::new(state.tasks().len())
        .viewport_content_length(height as usize)
        .position(state.focused_task_index);
    frame.render_stateful_widget(scrollbar, size, &mut scrollbar_state);
}
