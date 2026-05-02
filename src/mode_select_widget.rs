use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Paragraph, Widget},
};

pub struct ModeSelectWidget {
    wordset_index: usize,
    time_index: usize,
    wordset_options: Vec<String>,
    time_options: Vec<u32>,
}

impl std::fmt::Debug for ModeSelectWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModeSelectWidget")
            .field("wordset_index", &self.wordset_index)
            .field("time_index", &self.time_index)
            .finish()
    }
}

impl ModeSelectWidget {
    pub fn new(wordset_names: Vec<String>) -> Self {
        let times: Vec<u32> = (15..=300).step_by(15).collect();
        Self {
            wordset_index: 0,
            time_index: 0,
            wordset_options: wordset_names,
            time_options: times,
        }
    }

    pub fn wordset_index(&self) -> usize {
        self.wordset_index
    }

    pub fn time_index(&self) -> usize {
        self.time_index
    }

    pub fn selected_time(&self) -> u32 {
        self.time_options[self.time_index]
    }

    pub fn selected_wordset(&self) -> &str {
        &self.wordset_options[self.wordset_index]
    }

    pub fn move_left(&mut self) {
        if self.wordset_index > 0 {
            self.wordset_index -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.wordset_index < self.wordset_options.len() - 1 {
            self.wordset_index += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.time_index < self.time_options.len() - 1 {
            self.time_index += 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.time_index > 0 {
            self.time_index -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.wordset_index = 0;
        self.time_index = 0;
    }
}

impl Widget for &ModeSelectWidget {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let total_lines = 9;
        let start_y = area.y + (area.height - total_lines as u16) / 2;

        let title = Paragraph::new(Line::from("Select Mode").style(Style::default().fg(Color::Cyan)))
            .alignment(Alignment::Center);
        let title_area = Rect {
            x: area.x,
            y: start_y,
            width: area.width,
            height: 1,
        };
        title.render(title_area, buf);

        let wordset_label = Paragraph::new(Line::from("Wordset:").style(Style::default().fg(Color::White)))
            .alignment(Alignment::Center);
        let wordset_label_area = Rect {
            x: area.x,
            y: start_y + 2,
            width: area.width,
            height: 1,
        };
        wordset_label.render(wordset_label_area, buf);

        let wordset_value = Line::from(format!("< {} >", self.wordset_options[self.wordset_index])).style(Style::default().fg(Color::Yellow));
        let wordset_value_area = Rect {
            x: area.x,
            y: start_y + 3,
            width: area.width,
            height: 1,
        };
        Paragraph::new(wordset_value).alignment(Alignment::Center).render(wordset_value_area, buf);

        let time_label = Paragraph::new(Line::from("Time:").style(Style::default().fg(Color::White)))
            .alignment(Alignment::Center);
        let time_label_area = Rect {
            x: area.x,
            y: start_y + 5,
            width: area.width,
            height: 1,
        };
        time_label.render(time_label_area, buf);

        let seconds = self.time_options[self.time_index];
        let time_str = if seconds >= 60 {
            let min = seconds / 60;
            let sec = seconds % 60;
            if sec == 0 {
                format!("{} min", min)
            } else {
                format!("{} min {} sec", min, sec)
            }
        } else {
            format!("{} sec", seconds)
        };
        let time_value = Line::from(format!("< {} >", time_str)).style(Style::default().fg(Color::Yellow));
        let time_value_area = Rect {
            x: area.x,
            y: start_y + 6,
            width: area.width,
            height: 1,
        };
        Paragraph::new(time_value).alignment(Alignment::Center).render(time_value_area, buf);

        let hint = Paragraph::new(
            Line::from("← → : wordset  ↑ ↓ : time  Enter : start  Esc : back").style(Style::default().fg(Color::DarkGray))
        ).alignment(Alignment::Center);
        let hint_area = Rect {
            x: area.x,
            y: start_y + 8,
            width: area.width,
            height: 1,
        };
        hint.render(hint_area, buf);
    }
}