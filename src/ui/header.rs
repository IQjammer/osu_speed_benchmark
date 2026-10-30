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

    Paragraph::new("<Enter> to Start | <q> to Quit | <k> to change Keys")
        .centered()
        .block(block)
        .render(area, buf)
}
