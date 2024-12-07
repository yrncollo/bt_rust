use ratatui::{layout::Rect, style::{Color, Style, Stylize}, widgets::{Block, BorderType, Borders}, Frame};


pub fn devices_ui(frame: &mut Frame, rect: Rect) {
    
    let devices_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().blue())
        .title("Discoverd Devices");
        //.title_style(Style::default().fg(Color::Yellow));


    frame.render_widget(devices_block, rect);
}

pub fn paired_devices_ui(frame: &mut Frame, rect: Rect) {
    
    let devices_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().blue())
        .title("Paired Devices");
        //.title_style(Style::default().fg(Color::Yellow));

    frame.render_widget(devices_block, rect);
}

pub fn connected_devices_ui(frame: &mut Frame, rect: Rect) {
    
    let devices_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().blue())
        .title("Connected Devices")
       // .title_style(Style::default().fg(Color::Yellow))
        .title_alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(devices_block, rect);
}
