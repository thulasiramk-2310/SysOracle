use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table},
    Frame,
};

use crate::metrics::{Metrics, ProcInfo};

pub fn draw(
    frame: &mut Frame,
    metrics: &Metrics,
    procs: &[ProcInfo],
    alerts: &[String],       
    selected: usize,
) {
    let size = frame.size();

    // ── Main layout
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  
            Constraint::Length(4), 
            Constraint::Min(10),    
            Constraint::Length(4),  
            Constraint::Length(3),  
        ])
        .split(size);

    // ── CPU & MEM
    let cpu_mem = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(main[0]);

    draw_cpu(frame, cpu_mem[0], metrics);
    draw_mem(frame, cpu_mem[1], metrics);

    // ── Network
    frame.render_widget(
        Paragraph::new(" RX  ↓  2.4 MB/s\n TX  ↑  0.6 MB/s")
            .block(Block::default().title("Network").borders(Borders::ALL))
            .style(Style::default().fg(Color::Green)),
        main[1],
    );

    // ── Processes
    draw_processes(frame, main[2], procs, selected);

    // ── Alerts (LIVE)
    let alert_text = if alerts.is_empty() {
        " ✓ No active alerts".to_string()
    } else {
        alerts.join("\n")
    };

    frame.render_widget(
        Paragraph::new(alert_text)
            .block(Block::default().title("Alerts").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow)),
        main[3],
    );

    // ── Footer / Status Bar
    frame.render_widget(
        Paragraph::new(
            " MODE: PROC | ↑↓ Select | k Kill | r Reload | q Quit "
        )
        .block(
            Block::default()
                .title("Status")
                .borders(Borders::ALL),
        )
        .style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Rgb(20, 20, 20)),
        ),
        main[4],
    );
}



fn draw_cpu(frame: &mut Frame, area: Rect, metrics: &Metrics) {
    let cpu = metrics.cpu.clamp(0.0, 100.0) as u16;

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!("CPU {}%", cpu))
                .borders(Borders::ALL),
        )
        .gauge_style(
            Style::default().fg(if cpu > 80 {
                Color::Red
            } else if cpu > 60 {
                Color::Yellow
            } else {
                Color::Green
            }),
        )
        .percent(cpu);

    frame.render_widget(gauge, area);
}



fn draw_mem(frame: &mut Frame, area: Rect, metrics: &Metrics) {
    let percent =
        ((metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0)
            .clamp(0.0, 100.0) as u16;

    let used = metrics.memory_used / 1024 / 1024;
    let total = metrics.memory_total / 1024 / 1024;

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!("MEM {}% ({} / {} MB)", percent, used, total))
                .borders(Borders::ALL),
        )
        .gauge_style(
            Style::default().fg(if percent > 85 {
                Color::Red
            } else if percent > 65 {
                Color::Yellow
            } else {
                Color::Blue
            }),
        )
        .percent(percent);

    frame.render_widget(gauge, area);
}


fn draw_processes(
    frame: &mut Frame,
    area: Rect,
    procs: &[ProcInfo],
    selected: usize,
) {
    let header = Row::new(vec!["PID", "NAME", "CPU%", "MEM%"])
        .style(Style::default().fg(Color::Cyan));

    let rows = procs.iter().enumerate().map(|(i, p)| {
        let style = if i == selected {
            Style::default().fg(Color::Black).bg(Color::White)
        } else {
            Style::default()
        };

        Row::new(vec![
            p.pid.to_string(),
            p.name.clone(),
            format!("{:.1}", p.cpu),
            format!("{:.1}", p.mem),
        ])
        .style(style)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Length(28),
            Constraint::Length(8),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .block(Block::default().title("Processes").borders(Borders::ALL))
    .column_spacing(1);

    frame.render_widget(table, area);
}
