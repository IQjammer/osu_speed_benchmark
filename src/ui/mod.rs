mod bench;
mod header;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

use crate::app::App;

pub fn render(app: &App, area: Rect, buf: &mut Buffer) {
    let [header_area, content_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(area);

    header::render_header(header_area, buf);
    bench::render_bench(&app.bench, content_area, buf);
}
