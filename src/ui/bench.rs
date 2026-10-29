use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::bench::BenchmarkState;

pub fn render_bench(state: &BenchmarkState, area: Rect, buf: &mut Buffer) {
    let [top_center, bottom_keys] =
        Layout::vertical([Constraint::Length(5), Constraint::Min(3)]).areas(area);

    let status_line = if state.is_active {
        format!("Clicks: {} / {}", state.current_clicks, state.target_clicks)
    } else if state.current_clicks >= state.target_clicks && state.target_clicks > 0 {
        format!("Done! Total clicks: {}", state.current_clicks)
    } else {
        "Ready. Hit Enter to begin!".to_string()
    };

    Paragraph::new(status_line)
        .bold()
        .centered()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .render(top_center, buf);

    let [k1_area, k2_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(bottom_keys);

    let k1_active = state.last_key == Some('a');
    let k2_active = state.last_key == Some('s');

    let k1_block = Block::default()
        .borders(Borders::ALL)
        .title(" Key 2 (A) ")
        .border_style(if k2_active {
            ratatui::style::Style::default().bold()
        } else {
            ratatui::style::Style::default()
        });

    let k2_block = Block::default()
        .borders(Borders::ALL)
        .title(" Key 2 (S) ")
        .border_style(if k2_active {
            ratatui::style::Style::default().bold()
        } else {
            ratatui::style::Style::default()
        });

    Paragraph::new(if k1_active { "Pressed" } else { "" })
        .centered()
        .block(k1_block)
        .render(k1_area, buf);

    Paragraph::new(if k2_active { "PRESSED" } else { "" })
        .centered()
        .block(k2_block)
        .render(k2_area, buf);
}
