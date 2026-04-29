use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Paragraph, Widget},
};

#[derive(Debug)]
pub struct StatsWidget {
    wpm: f64,
    accuracy: f64,
    elapsed: f64,
}

impl StatsWidget {
    pub fn new() -> Self {
        Self {
            wpm: 0.0,
            accuracy: 100.0,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, wpm: f64, accuracy: f64, elapsed: f64) {
        self.wpm = wpm;
        self.accuracy = accuracy;
        self.elapsed = elapsed;
    }
}

impl Widget for &StatsWidget {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
            ])
            .spacing(2)
            .split(area);

        let time_str = format!("{:.1}s", self.elapsed);
        let wpm_str = format!("{:.0}", self.wpm);
        let acc_str = format!("{:.0}%", self.accuracy * 100.0);

        let wpm_line = Line::from(wpm_str).style(Style::default().fg(Color::Cyan));
        let acc_line = Line::from(acc_str).style(Style::default().fg(Color::Green));
        let time_line = Line::from(time_str).style(Style::default().fg(Color::Yellow));

        Paragraph::new(wpm_line).render(chunks[0], buf);
        Paragraph::new(acc_line).render(chunks[1], buf);
        Paragraph::new(time_line).render(chunks[2], buf);
    }
}