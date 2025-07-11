use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

/// Cache for fractal buffer
use std::{cell::RefCell, vec};
use tryptamine_core::math::fractal_definitions::{
    Compl, ComplexFatouFractal, FractalConfig, MandelbrotSet, RegularJuliaSet, SinJuliaSet,
};
use tryptamine_core::math::fractal_logic::ImageSchema;

/// Cache for fractal buffer and parameters
#[derive(Debug)]
pub struct FractalCache {
    pub res_x: u32,
    pub res_y: u32,
    pub window_diagonal: f64,
    pub center_cord: Compl,
    pub fractal_type: FractalConfig,
    pub buffer: Vec<u8>,
    pub times_rendered: u64,
}

const DEFAULT_DIAGONAL: f64 = 4.0;

const DEFAULT_CENTER: Compl = Compl::new(0.0, 0.0);

impl Default for FractalCache {
    fn default() -> Self {
        FractalCache {
            res_x: 100,
            res_y: 100,
            window_diagonal: DEFAULT_DIAGONAL,
            center_cord: Compl::default(),
            buffer: Vec::with_capacity(2000),
            fractal_type: FractalConfig::default(),
            times_rendered: 0,
        }
    }
}

/// Application state.
#[derive(Debug)]
pub struct App {
    /// Diagonal length for fractal rendering
    pub diagonal: f64,
    /// Real Center of Image
    pub real_center: f64,
    /// Imaginary Center of Image
    pub imag_center: f64,
    /// Available fractal implementations
    pub fractal_titles: Vec<&'static str>,
    /// Selected fractal index
    pub fractal_index: usize,
    /// Cache for fractal buffer
    pub fractal_cache: RefCell<FractalCache>,
    /// Is the application running?
    pub running: bool,
    /// Counter (unused for fractal but kept from template)
    pub counter: u64,
    /// Event handler
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            diagonal: 4.0,
            real_center: 2.0,
            imag_center: -1.5,
            fractal_titles: vec!["Mandelbrot", "Julia", "Sin Julia"],
            fractal_index: 0,
            fractal_cache: RefCell::new(FractalCache::default()),
            running: true,
            counter: 0,
            // window_diagonal: DEFAULT_DIAGONAL,
            // center_re: DEFAULT_CENTER.re,
            // center_im: DEFAULT_CENTER.im,
            events: EventHandler::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    /// Main loop
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Handle incoming events
    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(evt) => {
                if let crossterm::event::Event::Key(key) = evt {
                    self.handle_key_event(key)?;
                }
            }
            Event::App(app_evt) => match app_evt {
                AppEvent::Increment => self.increment_counter(),
                AppEvent::Decrement => self.decrement_counter(),
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handle key presses
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            // Quit application
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // Adjust diagonal length
            KeyCode::Char('w') => {
                self.diagonal += 0.1 * self.diagonal;
            }
            KeyCode::Char('s') => {
                self.diagonal = (self.diagonal * 0.9).max(0.1);
            }
            // Adjust real axis range with vim-like motions
            KeyCode::Char('l') => {
                self.real_center += 0.1 * self.diagonal;
            }
            KeyCode::Char('h') => {
                self.real_center -= 0.1 * self.diagonal;
            }
            // Adjust imaginary axis range with vim-like motions
            KeyCode::Char('j') => {
                self.imag_center += 0.1 * self.diagonal;
            }
            KeyCode::Char('k') => {
                self.imag_center -= 0.1 * self.diagonal;
            }
            // Cycle fractal selection
            KeyCode::Tab => {
                self.fractal_index = (self.fractal_index + 1) % self.fractal_titles.len();
            }
            // Counter controls (legacy)
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            _ => {}
        }
        Ok(())
    }

    /// Tick event
    pub fn tick(&self) {}

    /// Quit the app
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Increment counter
    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    /// Decrement counter
    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}
