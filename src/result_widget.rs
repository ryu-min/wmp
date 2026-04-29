use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Paragraph, Widget},
};

#[derive(Debug)]
pub struct ResultWidget {
    wpm: f64,
    accuracy: f64,
    time: f64,
    selected_index: usize,
}

impl ResultWidget {
    pub fn new() -> Self {
        Self {
            wpm: 0.0,
            accuracy: 0.0,
            time: 0.0,
            selected_index: 0,
        }
    }

    pub fn update(&mut self, wpm: f64, accuracy: f64, time: f64) {
        self.wpm = wpm;
        self.accuracy = accuracy;
        self.time = time;
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < 1 {
            self.selected_index += 1;
        }
    }

    pub fn reset(&mut self) {
        self.selected_index = 0;
    }
}

impl Widget for &ResultWidget {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let options = vec!["Restart", "Menu"];
        
        let wpm_line = Line::from(format!("WPM: {}", self.wpm as u32))
            .style(Style::default().fg(Color::Cyan));
        let acc_line = Line::from(format!("Accuracy: {:.0}%", self.accuracy * 100.0))
            .style(Style::default().fg(Color::Green));
        let time_line = Line::from(format!("Time: {:.1}s", self.time))
            .style(Style::default().fg(Color::Yellow));

        let stats_start_y = area.y + area.height.saturating_sub(1) / 2 - 4;

        let wpm_area = Rect {
            x: area.x,
            y: stats_start_y,
            width: area.width,
            height: 1,
        };
        let acc_area = Rect {
            x: area.x,
            y: stats_start_y + 1,
            width: area.width,
            height: 1,
        };
        let time_area = Rect {
            x: area.x,
            y: stats_start_y + 2,
            width: area.width,
            height: 1,
        };

        Paragraph::new(wpm_line).alignment(ratatui::layout::Alignment::Center).render(wpm_area, buf);
        Paragraph::new(acc_line).alignment(ratatui::layout::Alignment::Center).render(acc_area, buf);
        Paragraph::new(time_line).alignment(ratatui::layout::Alignment::Center).render(time_area, buf);

        let menu_start_y = stats_start_y + 4;

        for (i, option) in options.iter().enumerate() {
            let y = menu_start_y + i as u16;

            let line = if i == self.selected_index {
                Line::from(format!("> {}", option)).style(Style::default().fg(Color::Yellow))
            } else {
                Line::from(format!("  {}", option)).style(Style::default().fg(Color::White))
            };

            let option_area = Rect {
                x: area.x,
                y,
                width: area.width,
                height: 1,
            };

            Paragraph::new(line).alignment(ratatui::layout::Alignment::Center).render(option_area, buf);
        }
    }
}