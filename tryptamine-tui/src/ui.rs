use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Widget},
};

use crate::app::App;
use tryptamine_core::math::fractal_definitions::{
    Compl, FractalConfig,
};
use tryptamine_core::math::fractal_logic::{ImageSchema, generate_raw_image_buffer};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut cache = self.fractal_cache.borrow_mut();
        // Top-level block with border and title
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("tryptamine-tui")
            .title_alignment(Alignment::Center);
        block.render(area, buf);

        // Split into header and body
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .margin(1)
            .split(area);

        // Header: instructions and counter
        let header_text = format!(
            "This is a tui template.\n\
             Press `Esc`, `Ctrl-C` or `q` to quit.\n\
             Use keys to adjust settings (W/S: diagonal, R/F: real, I/K: imag, Tab: cycle fractal).\n\
             Times Rendered: {}",
            cache.times_rendered
        );
        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::ALL).title("Info"))
            .style(Style::default().fg(Color::Cyan));
        header.render(main_chunks[0], buf);

        // Body: split into controls and fractal render
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(main_chunks[1]);

        // Controls panel
        let control_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(body_chunks[0]);

        // Extract dynamic values from App
        let fractal_titles = &self.fractal_titles;
        let selected = self.fractal_index;

        // Diagonal length widget
        let diag_para = Paragraph::new(format!("{:.2}", self.diagonal))
            .block(Block::default().borders(Borders::ALL).title("Diagonal"));
        diag_para.render(control_chunks[0], buf);

        // Real range widget
        let real_para = Paragraph::new(format!("{:.2}", self.real_center)).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Real Center Value"),
        );
        real_para.render(control_chunks[1], buf);

        // Imag range widget
        let imag_para = Paragraph::new(format!("{:.2}", self.imag_center)).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Imaginary Center Value"),
        );
        imag_para.render(control_chunks[2], buf);

        // Fractal selection tabs
        let tabs = Tabs::new(
            fractal_titles
                .iter()
                .map(|t| Span::from(*t))
                .collect::<Vec<_>>(),
        )
        .block(Block::default().borders(Borders::ALL).title("Fractal"))
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow));
        tabs.render(control_chunks[3], buf);

        // Render fractal swatch
        let swatch = body_chunks[1];
        let w = swatch.width;
        let h = swatch.height;
        let res_x = w as u32;
        let res_y = h as u32;
        let pixel_ratio = 2.0; // TODO: Set this to the font ratio height/width

        // Check if we need to update buffer
        let needs_update = cache.res_x != res_x
            || cache.res_y != res_y
            || cache.fractal_type.get_index() as usize != self.fractal_index
            || (cache.center_cord.re - self.real_center).abs() > f64::EPSILON
            || (cache.center_cord.im - self.imag_center).abs() > f64::EPSILON
            || (cache.window_diagonal - self.diagonal).abs() > f64::EPSILON;
        if needs_update {
            cache.times_rendered += 1;
            cache.fractal_type =
                FractalConfig::default_from_index(self.fractal_index as u8).unwrap();
            cache.res_x = res_x;
            cache.res_y = res_y;
            cache.window_diagonal = self.diagonal;
            cache.center_cord.re = self.real_center;
            cache.center_cord.im = self.imag_center;
            // Build schema using values from App
            let schema = ImageSchema {
                resolution_x: res_x,
                resolution_y: res_y,
                pixel_ratio,
                window_diagonal: self.diagonal,
                center_cord: Compl::new(self.real_center, self.imag_center),
                ..Default::default()
            };
            // Choose appropriate fractal source (placeholder uses Mandelbrot for all)
            cache.buffer = generate_raw_image_buffer(&cache.fractal_type, &schema);
        }
        let buffer = &cache.buffer;

        for dy in 0..h {
            for dx in 0..w {
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
