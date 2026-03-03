use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{
        Axis, Block, Borders, Chart, Clear, Dataset, Gauge, GraphType, Paragraph, Row, Table,
    },
    Frame,
};

use crate::{
    app::{Theme, ViewMode},
    metrics::{Metrics, ProcInfo},
};

pub fn draw(
    frame: &mut Frame,
    metrics: &Metrics,
    procs: &[ProcInfo],
    alerts: &[String],
    selected: usize,
    view_mode: ViewMode,
    theme: Theme,
    cpu_history: &[f32],
    mem_history: &[f32],
    show_kill_popup: bool,
) {
    let (_bg_color, _fg_color, _accent_color) = match theme {
        Theme::Dark => (Color::Rgb(20, 20, 20), Color::White, Color::Cyan),
        Theme::Light => (Color::White, Color::Black, Color::Blue),
    };

    let _size = frame.size();

    match view_mode {
        ViewMode::All => {
            draw_all_view(frame, metrics, procs, alerts, selected, theme, cpu_history, mem_history)
        }
        ViewMode::Cpu => draw_cpu_view(frame, metrics, theme, cpu_history),
        ViewMode::Memory => draw_memory_view(frame, metrics, theme, mem_history),
        ViewMode::Process => draw_process_view(frame, procs, selected, theme),
    }

    // Kill confirmation popup
    if show_kill_popup {
        draw_kill_popup(frame, procs, selected, theme);
    }
}

fn draw_all_view(
    frame: &mut Frame,
    metrics: &Metrics,
    procs: &[ProcInfo],
    alerts: &[String],
    selected: usize,
    theme: Theme,
    cpu_history: &[f32],
    mem_history: &[f32],
) {
    let size = frame.size();

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(8),  // CPU & MEM Gauges + Graphs
            Constraint::Length(4),  // Network
            Constraint::Min(10),    // Processes
            Constraint::Length(5),  // Alerts
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    // Header
    draw_header(frame, main[0], ViewMode::All, theme);

    // CPU & MEM
    let metrics_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main[1]);

    draw_cpu_gauge_with_graph(frame, metrics_area[0], metrics, theme, cpu_history);
    draw_mem_gauge_with_graph(frame, metrics_area[1], metrics, theme, mem_history);

    // Network
    draw_network(frame, main[2], metrics, theme);

    // Processes
    draw_processes(frame, main[3], procs, selected, theme);

    // Alerts
    draw_alerts(frame, main[4], alerts, theme);

    // Footer
    draw_footer(frame, main[5], ViewMode::All, theme);
}

fn draw_cpu_view(frame: &mut Frame, metrics: &Metrics, theme: Theme, cpu_history: &[f32]) {
    let size = frame.size();

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // CPU Graph
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    draw_header(frame, main[0], ViewMode::Cpu, theme);
    draw_cpu_fullscreen(frame, main[1], metrics, theme, cpu_history);
    draw_footer(frame, main[2], ViewMode::Cpu, theme);
}

fn draw_memory_view(frame: &mut Frame, metrics: &Metrics, theme: Theme, mem_history: &[f32]) {
    let size = frame.size();

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Memory Graph
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    draw_header(frame, main[0], ViewMode::Memory, theme);
    draw_memory_fullscreen(frame, main[1], metrics, theme, mem_history);
    draw_footer(frame, main[2], ViewMode::Memory, theme);
}

fn draw_process_view(frame: &mut Frame, procs: &[ProcInfo], selected: usize, theme: Theme) {
    let size = frame.size();

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Processes
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    draw_header(frame, main[0], ViewMode::Process, theme);
    draw_processes(frame, main[1], procs, selected, theme);
    draw_footer(frame, main[2], ViewMode::Process, theme);
}

fn draw_header(frame: &mut Frame, area: Rect, mode: ViewMode, theme: Theme) {
    let (bg_color, fg_color) = match theme {
        Theme::Dark => (Color::Rgb(30, 30, 30), Color::Cyan),
        Theme::Light => (Color::Rgb(230, 230, 230), Color::Blue),
    };

    let mode_str = match mode {
        ViewMode::All => "ALL",
        ViewMode::Cpu => "CPU",
        ViewMode::Memory => "MEMORY",
        ViewMode::Process => "PROCESS",
    };

    let os_info = format!(
        "{} {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    let header_text = format!("  SysOracle v0.1.0  |  {}  |  Mode: {}", os_info, mode_str);

    let header = Paragraph::new(header_text)
        .style(Style::default().bg(bg_color).fg(fg_color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);

    frame.render_widget(header, area);
}

fn draw_cpu_gauge_with_graph(
    frame: &mut Frame,
    area: Rect,
    metrics: &Metrics,
    theme: Theme,
    cpu_history: &[f32],
) {
    let split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .split(area);

    let cpu = metrics.cpu.clamp(0.0, 100.0) as u16;
    let color = if cpu > 80 {
        Color::Red
    } else if cpu > 60 {
        Color::Yellow
    } else {
        Color::Green
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!("CPU {:.1}%", cpu))
                .borders(Borders::ALL)
                .border_style(get_border_style(theme)),
        )
        .gauge_style(Style::default().fg(color))
        .percent(cpu);

    frame.render_widget(gauge, split[0]);

    // Mini graph
    if cpu_history.len() > 1 {
        draw_mini_graph(frame, split[1], cpu_history, color, theme);
    }
}

fn draw_mem_gauge_with_graph(
    frame: &mut Frame,
    area: Rect,
    metrics: &Metrics,
    theme: Theme,
    mem_history: &[f32],
) {
    let split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .split(area);

    let percent = ((metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0)
        .clamp(0.0, 100.0) as u16;

    let used = metrics.memory_used / 1024 / 1024;
    let total = metrics.memory_total / 1024 / 1024;

    let color = if percent > 85 {
        Color::Red
    } else if percent > 65 {
        Color::Yellow
    } else {
        Color::Blue
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!("MEM {}% ({} / {} MB)", percent, used, total))
                .borders(Borders::ALL)
                .border_style(get_border_style(theme)),
        )
        .gauge_style(Style::default().fg(color))
        .percent(percent);

    frame.render_widget(gauge, split[0]);

    // Mini graph
    if mem_history.len() > 1 {
        draw_mini_graph(frame, split[1], mem_history, color, theme);
    }
}

fn draw_mini_graph(frame: &mut Frame, area: Rect, history: &[f32], color: Color, _theme: Theme) {
    let data: Vec<(f64, f64)> = history
        .iter()
        .enumerate()
        .map(|(i, &val)| (i as f64, val as f64))
        .collect();

    let dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(color))
        .data(&data);

    let x_max = history.len().max(2) as f64;

    let chart = Chart::new(vec![dataset])
        .x_axis(
            Axis::default()
                .bounds([0.0, x_max])
                .labels(vec![]),
        )
        .y_axis(
            Axis::default()
                .bounds([0.0, 100.0])
                .labels(vec![]),
        );

    frame.render_widget(chart, area);
}

fn draw_cpu_fullscreen(
    frame: &mut Frame,
    area: Rect,
    metrics: &Metrics,
    theme: Theme,
    cpu_history: &[f32],
) {
    let cpu = metrics.cpu.clamp(0.0, 100.0);
    let color = if cpu > 80.0 {
        Color::Red
    } else if cpu > 60.0 {
        Color::Yellow
    } else {
        Color::Green
    };

    if cpu_history.len() > 1 {
        let data: Vec<(f64, f64)> = cpu_history
            .iter()
            .enumerate()
            .map(|(i, &val)| (i as f64, val as f64))
            .collect();

        let dataset = Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(color))
            .data(&data);

        let x_max = cpu_history.len().max(2) as f64;

        let chart = Chart::new(vec![dataset])
            .block(
                Block::default()
                    .title(format!("CPU Usage: {:.1}%", cpu))
                    .borders(Borders::ALL)
                    .border_style(get_border_style(theme)),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .bounds([0.0, x_max])
                    .labels(vec!["".into(), "".into()]),
            )
            .y_axis(
                Axis::default()
                    .title("Usage %")
                    .bounds([0.0, 100.0])
                    .labels(vec!["0%".into(), "50%".into(), "100%".into()]),
            );

        frame.render_widget(chart, area);
    }
}

fn draw_memory_fullscreen(
    frame: &mut Frame,
    area: Rect,
    metrics: &Metrics,
    theme: Theme,
    mem_history: &[f32],
) {
    let mem_percent = ((metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0) as f32;
    let color = if mem_percent > 85.0 {
        Color::Red
    } else if mem_percent > 65.0 {
        Color::Yellow
    } else {
        Color::Blue
    };

    if mem_history.len() > 1 {
        let data: Vec<(f64, f64)> = mem_history
            .iter()
            .enumerate()
            .map(|(i, &val)| (i as f64, val as f64))
            .collect();

        let dataset = Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(color))
            .data(&data);

        let x_max = mem_history.len().max(2) as f64;

        let chart = Chart::new(vec![dataset])
            .block(
                Block::default()
                    .title(format!(
                        "Memory Usage: {:.1}% ({} / {} MB)",
                        mem_percent,
                        metrics.memory_used / 1024 / 1024,
                        metrics.memory_total / 1024 / 1024
                    ))
                    .borders(Borders::ALL)
                    .border_style(get_border_style(theme)),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .bounds([0.0, x_max])
                    .labels(vec!["".into(), "".into()]),
            )
            .y_axis(
                Axis::default()
                    .title("Usage %")
                    .bounds([0.0, 100.0])
                    .labels(vec!["0%".into(), "50%".into(), "100%".into()]),
            );

        frame.render_widget(chart, area);
    }
}

fn draw_network(frame: &mut Frame, area: Rect, metrics: &Metrics, theme: Theme) {
    let rx_mb = metrics.network_rx as f64 / 1024.0 / 1024.0;
    let tx_mb = metrics.network_tx as f64 / 1024.0 / 1024.0;

    let text = format!(
        " RX  ↓  {:.2} MB total\n TX  ↑  {:.2} MB total",
        rx_mb, tx_mb
    );

    frame.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .title("Network")
                    .borders(Borders::ALL)
                    .border_style(get_border_style(theme)),
            )
            .style(Style::default().fg(Color::Green)),
        area,
    );
}

fn draw_processes(
    frame: &mut Frame,
    area: Rect,
    procs: &[ProcInfo],
    selected: usize,
    theme: Theme,
) {
    let (fg_color, selected_bg) = match theme {
        Theme::Dark => (Color::White, Color::Rgb(60, 60, 60)),
        Theme::Light => (Color::Black, Color::Rgb(200, 200, 200)),
    };

    let header = Row::new(vec!["PID", "NAME", "CPU%", "MEM%"])
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let rows = procs.iter().enumerate().map(|(i, p)| {
        let style = if i == selected {
            Style::default().bg(selected_bg).fg(fg_color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(fg_color)
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
            Constraint::Min(28),
            Constraint::Length(8),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Processes (↑↓ navigate | k kill)")
            .borders(Borders::ALL)
            .border_style(get_border_style(theme)),
    )
    .column_spacing(2);

    frame.render_widget(table, area);
}

fn draw_alerts(frame: &mut Frame, area: Rect, alerts: &[String], theme: Theme) {
    let alert_text = if alerts.is_empty() {
        " ✓ No active alerts".to_string()
    } else {
        alerts.join("\n ")
    };

    let alert_color = if alerts.is_empty() {
        Color::Green
    } else {
        Color::Yellow
    };

    frame.render_widget(
        Paragraph::new(format!(" {}", alert_text))
            .block(
                Block::default()
                    .title("Alerts")
                    .borders(Borders::ALL)
                    .border_style(get_border_style(theme)),
            )
            .style(Style::default().fg(alert_color)),
        area,
    );
}

fn draw_footer(frame: &mut Frame, area: Rect, mode: ViewMode, theme: Theme) {
    let (bg_color, fg_color) = match theme {
        Theme::Dark => (Color::Rgb(30, 30, 30), Color::Cyan),
        Theme::Light => (Color::Rgb(230, 230, 230), Color::Blue),
    };

    let keys = match mode {
        ViewMode::All => "a All | c CPU | m Memory | p Process | t Theme | q Quit",
        ViewMode::Cpu => "a All | c CPU | m Memory | p Process | t Theme | q Quit",
        ViewMode::Memory => "a All | c CPU | m Memory | p Process | t Theme | q Quit",
        ViewMode::Process => "a All | ↑↓ Select | k Kill | t Theme | q Quit",
    };

    let footer = Paragraph::new(format!("  {}", keys))
        .style(Style::default().bg(bg_color).fg(fg_color))
        .alignment(Alignment::Left);

    frame.render_widget(footer, area);
}

fn draw_kill_popup(frame: &mut Frame, procs: &[ProcInfo], selected: usize, theme: Theme) {
    let (bg_color, fg_color, border_color) = match theme {
        Theme::Dark => (Color::Rgb(40, 40, 40), Color::White, Color::Red),
        Theme::Light => (Color::Rgb(240, 240, 240), Color::Black, Color::Red),
    };

    let area = centered_rect(50, 30, frame.size());

    frame.render_widget(Clear, area);

    let proc_name = procs
        .get(selected)
        .map(|p| p.name.as_str())
        .unwrap_or("Unknown");

    let proc_pid = procs.get(selected).map(|p| p.pid).unwrap_or(0);

    let text = format!(
        "\n Kill Process?\n\n Process: {}\n PID: {}\n\n Press Y to confirm, N to cancel",
        proc_name, proc_pid
    );

    let popup = Paragraph::new(text)
        .block(
            Block::default()
                .title("⚠ Confirmation")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color).add_modifier(Modifier::BOLD)),
        )
        .style(Style::default().bg(bg_color).fg(fg_color))
        .alignment(Alignment::Center);

    frame.render_widget(popup, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn get_border_style(theme: Theme) -> Style {
    match theme {
        Theme::Dark => Style::default().fg(Color::Rgb(80, 80, 80)),
        Theme::Light => Style::default().fg(Color::Rgb(150, 150, 150)),
    }
}