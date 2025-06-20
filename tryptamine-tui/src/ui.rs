use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;
use tryptamine_core::math::fractal_definitions::MandelbrotSet;
use tryptamine_core::math::fractal_logic::{ImageSchema, generate_raw_image_buffer};

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
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        // Render the text
        paragraph.render(chunks[0], buf);

        // Render Mandelbrot fractal in low resolution
        let swatch = chunks[1];
        let w = swatch.width;
        let h = swatch.height;

        // Configure low-resolution image schema
        let mut image_info = ImageSchema::default();
        let res_x = w as u32;
        let res_y = h as u32;
        image_info.resolution_x = res_x;
        image_info.resolution_y = res_y;
        image_info.pixel_ratio = 2.0;
        let mandelbrot = MandelbrotSet::default();
        let buffer = generate_raw_image_buffer(&mandelbrot, &image_info);

        for dy in 0..h {
            for dx in 0..w {
                // Compute buffer index for RGB triplet
                let idx = (((dy as u32) * res_x + (dx as u32)) * 3) as usize;
                let r = buffer[idx];
                let g = buffer[idx + 1];
                let b = buffer[idx + 2];
                buf.get_mut(swatch.x + dx, swatch.y + dy)
                    .set_bg(Color::Rgb(r, g, b))
                    .set_symbol(" ");
            }
        }
    }
}
