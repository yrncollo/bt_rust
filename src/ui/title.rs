use ratatui::{layout::Rect, style::{Style, Styled, Stylize}, text::Text, widgets::{Block, BorderType, Borders, Paragraph}, Frame};


pub fn title_ui(frame: &mut Frame, rect: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().blue());

    let title = Paragraph::new(
        Text::styled("BT, Connect With Ease", Style::default().fg(ratatui::style::Color::Green)))
        .centered()
        .block(title_block);

    frame.render_widget(title, rect);
}
