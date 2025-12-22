use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::metrics::Metrics;

pub fn draw(frame: &mut Frame, metrics: &Metrics) {
    let size = frame.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // CPU
            Constraint::Length(3), // Memory
        ])
        .split(size);

    // Header
    let header = Paragraph::new(" SysOracle 🔮  |  Press Q to quit ")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, layout[0]);

    // CPU Gauge
    let cpu = Gauge::default()
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red))
        .percent(metrics.cpu as u16);
    frame.render_widget(cpu, layout[1]);

    // Memory Gauge
    let mem_percent =
        ((metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0) as u16;

    let memory = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .percent(mem_percent);
    frame.render_widget(memory, layout[2]);
}
