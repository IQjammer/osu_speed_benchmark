use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

pub fn render_header(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .border_set(border::THICK)
        .title(" Osu!Speed Benchmark".bold());

    Paragraph::new("Press <Enter> to start | <q> to Quit | <r> to Reset")
        .centered()
        .block(block)
        .render(area, buf)
}
