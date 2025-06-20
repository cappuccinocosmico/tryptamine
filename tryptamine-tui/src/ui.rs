use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect, Layout, Direction, Constraint},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Main block with border and title
        let block = Block::bordered()
            .title("tryptamine-tui")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        // Counter text and instructions
        let text = format!(
            "This is a tui template.\n\
             Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
             Press left and right to increment and decrement the counter respectively.\n\
             Counter: {}",
            self.counter
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        // Layout: top for text, bottom for RGB swatch
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(4), Constraint::Length(3)])
            .split(area);

        // Render the text
        paragraph.render(chunks[0], buf);

        // Render RGB swatch beneath the counter
        let swatch = chunks[1];
        let x0 = swatch.x;
        let y0 = swatch.y;
        let w = swatch.width;
        let h = swatch.height;
        let third = w / 3;

        for dy in 0..h {
            for dx in 0..w {
                // Determine color segment (R, G, or B) based on horizontal position
                let color = if dx < third {
                    Color::Rgb(self.counter, 0, 0)
                } else if dx < third * 2 {
                    Color::Rgb(0, self.counter, 0)
                } else {
                    Color::Rgb(0, 0, self.counter)
                };

                buf.get_mut(x0 + dx, y0 + dy)
                    .set_bg(color)
                    .set_symbol(" ");
            }
        }
    }
}
