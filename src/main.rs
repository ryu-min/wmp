mod typing_widget;
mod stats_widget;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use typing_widget::TypingWidget;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug)]
pub struct App {
    running: bool,
    typing_widget: TypingWidget,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            typing_widget: TypingWidget::new(
                "The quick brown fox jumps over the lazy dog The quick brown fox jumps over the lazy dog The quick brown fox jumps over the lazy dog".to_string(),
            ),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        self.typing_widget.update_stats();
        frame.render_widget(&self.typing_widget, frame.area());
    }

    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.quit(),
            KeyCode::Char(ch) => {
                self.typing_widget.add_char(ch);
            }
            KeyCode::Backspace => {
                self.typing_widget.remove_char();
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}