use ratatui::{layout::{Constraint, Direction, Layout, Margin, Rect}, style::{Color, Style}, widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState}, Frame};

use super::{TaskFocus, UserInterfaceState};

pub fn render<'a>(frame: &mut Frame, state: &mut UserInterfaceState<'a>) {
    let size = frame.size();
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"))
        .style(Color::Cyan);
    let height = size.height;
    let selected_index = state.selected_index();
    let focus_type = state.focus_type();
    let task_count = state.tasks_count();
    let focused_task_index = state.focused_task_index;
    for (index, task) in state.tasks_mut().enumerate() {
        let selected = selected_index == index;
        let [task_layout, time_layout] = Layout::new(Direction::Vertical, [
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(Rect::new(size.x, size.y, size.width - 1, height));
        let default_color = if selected { Color::Cyan } else { Color::LightBlue };
        let scroll = (index as u16, 0);
        /*task.content.set_block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(if focus_type == TaskFocus::Content { Color::LightYellow }  else { default_color })
                .title(task.title.value())
                .title_style(if focus_type == TaskFocus::Title { Color::LightYellow }  else { default_color }),
        );
        //task.content.scroll(scroll);
        frame.render_widget(task.content.widget(), task_layout);
        task.time.set_block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(if focus_type == TaskFocus::Time { Color::LightYellow } else { default_color })
                .title("Time"),
        );
        //task.time.scroll(scroll);
        frame.render_widget(task.time.widget(), time_layout);*/
    }
    let mut scrollbar_state = ScrollbarState::new(task_count)
        .viewport_content_length(height as usize)
        .position(focused_task_index);
    frame.render_stateful_widget(scrollbar, size, &mut scrollbar_state);
}
