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

impl TypingWidget {
    fn split_text_into_lines(&self, text: &str, max_width: usize) -> Vec<(usize, usize)> {
        let mut lines = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut start = 0;
        
        while start < chars.len() {
            let mut end = start + max_width.min(chars.len() - start);
            
            if end < chars.len() {
                let mut last_space = end;
                for i in (start..end).rev() {
                    if chars[i].is_whitespace() {
                        last_space = i + 1;
                        break;
                    }
                }
                if last_space > start {
                    end = last_space;
                }
            }
            
            lines.push((start, end));
            start = end;
        }
        
        lines
    }
    
    fn get_current_line_index(&self, line_ranges: &[(usize, usize)], input_len: usize) -> usize {
        for (idx, &(start, end)) in line_ranges.iter().enumerate() {
            if input_len >= start && input_len <= end {
                if input_len == end && idx + 1 < line_ranges.len() {
                    return idx + 1;
                }
                return idx;
            }
        }
        line_ranges.len().saturating_sub(1)
    }
}

impl Widget for &TypingWidget {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
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

        let text_area = horizontal[1];
        let available_width = text_area.width.saturating_sub(2) as usize;
        
        if available_width == 0 {
            return;
        }

        let line_ranges = self.split_text_into_lines(&self.target_text, available_width);
        
        if line_ranges.is_empty() {
            return;
        }

        let input_len = self.input_text.chars().count();
        let current_line_idx = self.get_current_line_index(&line_ranges, input_len);
        let current_line_range = line_ranges[current_line_idx];
        
        let target_chars: Vec<char> = self.target_text.chars().collect();
        let input_chars: Vec<char> = self.input_text.chars().collect();
        
        let line_start = current_line_range.0;
        let line_end = current_line_range.1;
        let line_length = line_end - line_start;
        
        let input_in_line = if input_len > line_start {
            input_len.min(line_end) - line_start
        } else {
            0
        };
        
        let mut lines = Vec::new();
        lines.push(Line::from(vec![Span::raw("")]));
        lines.push(Line::from(vec![Span::raw("")]));
        
        let target_spans: Vec<Span> = (0..line_length)
            .map(|i| {
                let global_idx = line_start + i;
                if global_idx < target_chars.len() {
                    let target_char = target_chars[global_idx];
                    if global_idx < input_chars.len() {
                        let input_char = input_chars[global_idx];
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
                    } else if global_idx == input_chars.len() {
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
        
        let input_spans: Vec<Span> = (0..line_length)
            .map(|i| {
                let global_idx = line_start + i;
                if i < input_in_line && global_idx < input_chars.len() {
                    let ch = input_chars[global_idx];
                    if global_idx < target_chars.len() && ch == target_chars[global_idx] {
                        Span::styled(
                            ch.to_string(),
                            Style::default().fg(Color::Green),
                        )
                    } else if global_idx < target_chars.len() {
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
        
        let target_line = Line::from(target_spans);
        lines.push(target_line);
        lines.push(Line::from(vec![Span::raw("")]));
        
        let input_line = Line::from(input_spans);
        lines.push(input_line);
        lines.push(Line::from(vec![Span::raw("")]));
        lines.push(Line::from(vec![Span::raw("")]));

        let text_width = line_length.min(available_width);
        let centered_area = if text_width < text_area.width as usize {
            let offset = (text_area.width as usize - text_width) / 2;
            Rect {
                x: text_area.x + offset as u16,
                y: text_area.y,
                width: text_width as u16,
                height: text_area.height,
            }
        } else {
            text_area
        };
        
        let paragraph = Paragraph::new(lines)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        
        paragraph.render(centered_area, buf);
    }
}

