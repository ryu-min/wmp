use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

#[derive(Debug)]
pub struct TypingWidget {
    target_text: String,
    input_text: String,
}

impl TypingWidget {
    pub fn new(target_text: String) -> Self {
        Self {
            target_text,
            input_text: String::new(),
        }
    }

    pub fn with_target_text(mut self, target_text: String) -> Self {
        self.target_text = target_text;
        self.input_text.clear();
        self
    }

    pub fn set_target_text(&mut self, target_text: String) {
        self.target_text = target_text;
        self.input_text.clear();
    }

    pub fn add_char(&mut self, ch: char) {
        self.input_text.push(ch);
    }

    pub fn remove_char(&mut self) {
        self.input_text.pop();
    }

    pub fn reset(&mut self) {
        self.input_text.clear();
    }

    pub fn get_input(&self) -> &str {
        &self.input_text
    }

    pub fn is_complete(&self) -> bool {
        self.input_text == self.target_text
    }

    pub fn get_accuracy(&self) -> f64 {
        if self.target_text.is_empty() {
            return 1.0;
        }
        let correct = self
            .input_text
            .chars()
            .zip(self.target_text.chars())
            .filter(|(a, b)| a == b)
            .count();
        let total = self.input_text.len().max(self.target_text.len());
        if total == 0 {
            1.0
        } else {
            correct as f64 / total as f64
        }
    }
}

impl Widget for &TypingWidget {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut lines = Vec::new();

        let target_chars: Vec<char> = self.target_text.chars().collect();
        let input_chars: Vec<char> = self.input_text.chars().collect();


        let max_len = target_chars.len().max(input_chars.len());
        
        let input_spans: Vec<Span> = (0..max_len)
            .map(|i| {
                if i < input_chars.len() {
                    let ch = input_chars[i];
                    if i < target_chars.len() && ch == target_chars[i] {
                        Span::styled(
                            ch.to_string(),
                            Style::default().fg(Color::Green),
                        )
                    } else if i < target_chars.len() {
                        Span::styled(
                            ch.to_string(),
                            Style::default()
                                .fg(Color::Red)
                                .add_modifier(Modifier::UNDERLINED),
                        )
                    } else {
                        Span::styled(
                            ch.to_string(),
                            Style::default()
                                .fg(Color::Red)
                                .add_modifier(Modifier::UNDERLINED),
                        )
                    }
                } else {
                    Span::raw(" ")
                }
            })
            .collect();
        
        let target_spans: Vec<Span> = (0..max_len)
            .map(|i| {
                if i < target_chars.len() {
                    let target_char = target_chars[i];
                    if i < input_chars.len() {
                        let input_char = input_chars[i];
                        if input_char == target_char {
                            Span::styled(
                                target_char.to_string(),
                                Style::default().fg(Color::Green),
                            )
                        } else {
                            Span::styled(
                                target_char.to_string(),
                                Style::default()
                                    .fg(Color::Red)
                                    .add_modifier(Modifier::CROSSED_OUT),
                            )
                        }
                    } else if i == input_chars.len() {
                        Span::styled(
                            target_char.to_string(),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        )
                    } else {
                        Span::styled(
                            target_char.to_string(),
                            Style::default().fg(Color::White),
                        )
                    }
                } else {
                    Span::raw(" ")
                }
            })
            .collect();
        
        let target_line = Line::from(target_spans);
        lines.push(Line::from(vec![Span::raw("")]));
        lines.push(Line::from(vec![Span::raw("")]));
        lines.push(target_line);
        lines.push(Line::from(vec![Span::raw("")]));
        
        let input_line = Line::from(input_spans);
        lines.push(input_line);
        lines.push(Line::from(vec![Span::raw("")]));
        lines.push(Line::from(vec![Span::raw("")]));

        let block = Block::bordered().title("Typing Test");
        let inner_area = block.inner(area);
        block.render(area, buf);

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ])
            .split(inner_area);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(3),
                Constraint::Percentage(94),
                Constraint::Percentage(3),
            ])
            .split(vertical[1]);

        let paragraph = Paragraph::new(lines)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        
        let text_width = max_len.min(horizontal[1].width as usize);
        let text_area = if text_width < horizontal[1].width as usize {
            let offset = (horizontal[1].width as usize - text_width) / 2;
            Rect {
                x: horizontal[1].x + offset as u16,
                y: horizontal[1].y,
                width: text_width as u16,
                height: horizontal[1].height,
            }
        } else {
            horizontal[1]
        };
        
        paragraph.render(text_area, buf);
    }
}

