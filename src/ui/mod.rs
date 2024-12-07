mod title;
mod search_bar;
mod devices;


use devices::{connected_devices_ui, devices_ui, paired_devices_ui};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Style, Styled, Stylize}, text::Text, widgets::{Block, BorderType, Borders, Paragraph}, Frame};
use search_bar::{scan_ui, search_ui};
use title::title_ui;

use crate::app::App;

pub fn render_ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(100)
        ]).split(frame.area());

    let outer_container = Block::default()
        .border_style(Style::default().fg(ratatui::style::Color::Yellow))
        .borders(Borders::ALL);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(7)
        ]).split(chunks[0]);

    frame.render_widget(outer_container, chunks[0]);
    title_ui(frame, inner_chunks[0]);
    render_inner_block(frame, inner_chunks[1]);
    render_second_block(frame, inner_chunks[2]);
    connected_devices_ui(frame, inner_chunks[3]);
}

///This creates the inner box for search and scan part
fn render_inner_block(frame: &mut Frame, rect: Rect){
    
   let inner_box = Layout::new(
        Direction::Horizontal,
        vec![
            Constraint::Percentage(40),
            Constraint::Percentage(35),
            Constraint::Percentage(25),
        ]

        )
       .split(rect);
   
   search_ui(frame, inner_box[0]);
   scan_ui(frame, inner_box[2]);
}


/// This would render discovered devices and paired devices
fn render_second_block(frame: &mut Frame, rect: Rect) {
    let inner_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).split(rect);
    
    devices_ui(frame, inner_box[0]);
    paired_devices_ui(frame, inner_box[1]);
}

