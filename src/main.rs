mod menu_widget;
mod result_widget;
mod typing_widget;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use menu_widget::MenuWidget;
use ratatui::{DefaultTerminal, Frame};
use result_widget::ResultWidget;
use std::time::Duration;
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
    screen: Screen,
    menu_widget: MenuWidget,
    typing_widget: TypingWidget,
    result_widget: ResultWidget,
}

#[derive(Debug, PartialEq)]
enum Screen {
    Menu,
    Typing,
    Result,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            screen: Screen::Menu,
            menu_widget: MenuWidget::new(),
            typing_widget: TypingWidget::new(
                "The quick brown fox jumps over the lazy dog".to_string(),
            ),
            result_widget: ResultWidget::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            if self.screen == Screen::Typing {
                self.typing_widget.update_stats();
            }
            terminal.draw(|frame| self.render(frame))?;
            
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.on_key_event(key);
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        match self.screen {
            Screen::Menu => frame.render_widget(&self.menu_widget, frame.area()),
            Screen::Typing => frame.render_widget(&self.typing_widget, frame.area()),
            Screen::Result => frame.render_widget(&self.result_widget, frame.area()),
        }
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => self.quit(),
            _ => {
                match self.screen {
                    Screen::Menu => {
                        if key.code == KeyCode::Esc {
                            self.quit();
                        } else {
                            self.on_menu_key_event(key);
                        }
                    }
                    Screen::Typing => self.on_typing_key_event(key),
                    Screen::Result => self.on_result_key_event(key),
                }
            }
        }
    }

    fn on_menu_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.menu_widget.move_up(),
            KeyCode::Down => self.menu_widget.move_down(),
            KeyCode::Enter => {
                if self.menu_widget.selected_index() == 0 {
                    self.screen = Screen::Typing;
                } else {
                    self.quit();
                }
            }
            _ => {}
        }
    }

    fn on_typing_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(ch) => {
                self.typing_widget.add_char(ch);
            }
            KeyCode::Backspace => {
                self.typing_widget.remove_char();
            }
            KeyCode::Esc => {
                self.screen = Screen::Menu;
                self.typing_widget.reset();
                self.menu_widget.reset();
            }
            _ => {}
        }
        
        self.typing_widget.update_stats();
        
        if self.typing_widget.is_complete() {
            self.result_widget.update(
                self.typing_widget.get_wpm(),
                self.typing_widget.get_accuracy(),
                self.typing_widget.get_elapsed_time(),
            );
            self.screen = Screen::Result;
        }
    }

    fn on_result_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.result_widget.move_up(),
            KeyCode::Down => self.result_widget.move_down(),
            KeyCode::Enter => {
                if self.result_widget.selected_index() == 0 {
                    self.typing_widget.reset();
                    self.screen = Screen::Typing;
                } else {
                    self.screen = Screen::Menu;
                    self.typing_widget.reset();
                    self.menu_widget.reset();
                }
            }
            KeyCode::Esc => {
                self.screen = Screen::Menu;
                self.typing_widget.reset();
                self.menu_widget.reset();
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}