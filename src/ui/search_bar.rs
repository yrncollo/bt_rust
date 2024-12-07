use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, text::Text, widgets::{Block, BorderType, Borders, Paragraph}, Frame};

pub fn search_ui(frame: &mut Frame, rect: Rect) {
    
    let search_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().blue())
        .title("Search_Devices")
        .title_alignment(ratatui::layout::Alignment::Center)
        //.title_style(Style::default().fg(Color::Yellow))
        ;


    let title = Paragraph::new(
        Text::styled("Search bar", Style::default().fg(ratatui::style::Color::Green)))
        .centered()
        .block(search_block);

    frame.render_widget(title, rect);
}

pub fn scan_ui(frame: &mut Frame, rect: Rect) {
    let scan_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().blue())
        .title("Search_Devices")
        .title_alignment(ratatui::layout::Alignment::Center)
        //.title_style(Style::default().fg(Color::Yellow))
        .border_type(BorderType::Rounded);

    let placeholder = Paragraph::new("Scan/Rescan")
        .style(Style::default().fg(Color::Gray)) // Gray for placeholder text
        .centered()
        .block(scan_block); // Add the block to the paragraph

    frame.render_widget(placeholder, rect);
    
}
