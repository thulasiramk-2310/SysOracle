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
    app::{SortMode, Theme, ViewMode},
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
    sort_mode: SortMode,
    cpu_history: &[f32],
    mem_history: &[f32],
    show_kill_popup: bool,
) {
    let (bg_color, _fg_color, _accent_color) = match theme {
        Theme::Dark => (Color::Rgb(20, 20, 20), Color::White, Color::Cyan),
        Theme::Light => (Color::White, Color::Black, Color::Blue),
    };

    let size = frame.size();

    // Clear entire frame with background to prevent artifacts
    let background = Block::default()
        .style(Style::default().bg(bg_color));
    frame.render_widget(background, size);

    // Render exactly one view mode - strict match prevents overlapping
    match view_mode {
        ViewMode::All => {
            draw_all_view(frame, metrics, procs, alerts, selected, theme, sort_mode, cpu_history, mem_history)
        }
        ViewMode::Cpu => draw_cpu_view(frame, metrics, theme, cpu_history),
        ViewMode::Memory => draw_memory_view(frame, metrics, theme, mem_history),
        ViewMode::Process => draw_process_view(frame, procs, selected, theme, sort_mode),
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
    sort_mode: SortMode,
    cpu_history: &[f32],
    mem_history: &[f32],
) {
    let size = frame.size();
    let height = size.height;

    // Responsive layout based on terminal height
    if height >= 40 {
        // FULL MODE: Show all panels
        draw_all_view_full(frame, metrics, procs, alerts, selected, theme, sort_mode, cpu_history, mem_history);
    } else if height >= 25 {
        // COMPACT MODE: Reduce some panels
        draw_all_view_compact(frame, metrics, procs, alerts, selected, theme, sort_mode, cpu_history, mem_history);
    } else {
        // MINIMAL MODE: Show only essential info
        draw_all_view_minimal(frame, metrics, procs, selected, theme, sort_mode);
    }
}

fn draw_all_view_full(
    frame: &mut Frame,
    metrics: &Metrics,
    procs: &[ProcInfo],
    alerts: &[String],
    selected: usize,
    theme: Theme,
    sort_mode: SortMode,
    cpu_history: &[f32],
    mem_history: &[f32],
) {
    let size = frame.size();

    // Conditionally adjust layout based on GPU availability
    let has_gpu = metrics.gpu.is_some();
    
    let main = if has_gpu {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Length(8),  // CPU & MEM Gauges + Graphs
                Constraint::Length(4),  // Network
                Constraint::Length(4),  // GPU
                Constraint::Length(6),  // Disks
                Constraint::Min(8),     // Processes
                Constraint::Length(5),  // Alerts
                Constraint::Length(3),  // Footer
            ])
            .split(size)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Length(8),  // CPU & MEM Gauges + Graphs
                Constraint::Length(4),  // Network
                Constraint::Length(6),  // Disks
                Constraint::Min(8),     // Processes
                Constraint::Length(5),  // Alerts
                Constraint::Length(3),  // Footer
            ])
            .split(size)
    };

    // Header
    draw_header(frame, main[0], ViewMode::All, theme, metrics.uptime);

    // CPU & MEM
    let metrics_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main[1]);

    draw_cpu_gauge_with_graph(frame, metrics_area[0], metrics, theme, cpu_history);
    draw_mem_gauge_with_graph(frame, metrics_area[1], metrics, theme, mem_history);

    // Network
    draw_network(frame, main[2], metrics, theme);

    // GPU and subsequent panels - offset based on GPU presence
    if has_gpu {
        draw_gpu(frame, main[3], metrics, theme);
        draw_disks(frame, main[4], metrics, theme);
        draw_processes(frame, main[5], procs, selected, theme, sort_mode);
        draw_alerts(frame, main[6], alerts, theme);
        draw_footer(frame, main[7], ViewMode::All, theme);
    } else {
        draw_disks(frame, main[3], metrics, theme);
        draw_processes(frame, main[4], procs, selected, theme, sort_mode);
        draw_alerts(frame, main[5], alerts, theme);
        draw_footer(frame, main[6], ViewMode::All, theme);
    }
}

fn draw_all_view_compact(
    frame: &mut Frame,
    metrics: &Metrics,
    procs: &[ProcInfo],
    alerts: &[String],
    selected: usize,
    theme: Theme,
    sort_mode: SortMode,
    cpu_history: &[f32],
    mem_history: &[f32],
) {
    let size = frame.size();
    let has_gpu = metrics.gpu.is_some();

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(8),  // CPU & MEM Gauges + Graphs
            Constraint::Length(4),  // Network & GPU combined
            Constraint::Length(5),  // Disks (smaller)
            Constraint::Min(6),     // Processes
            Constraint::Length(3),  // Alerts (smaller)
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    // Header
    draw_header(frame, main[0], ViewMode::All, theme, metrics.uptime);

    // CPU & MEM
    let metrics_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main[1]);

    draw_cpu_gauge_with_graph(frame, metrics_area[0], metrics, theme, cpu_history);
    draw_mem_gauge_with_graph(frame, metrics_area[1], metrics, theme, mem_history);

    // Network & GPU side by side (or just Network if no GPU)
    if has_gpu {
        let net_gpu_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main[2]);

        draw_network(frame, net_gpu_area[0], metrics, theme);
        draw_gpu(frame, net_gpu_area[1], metrics, theme);
    } else {
        draw_network(frame, main[2], metrics, theme);
    }

    // Disks
    draw_disks(frame, main[3], metrics, theme);

    // Processes
    draw_processes(frame, main[4], procs, selected, theme, sort_mode);

    // Alerts
    draw_alerts(frame, main[5], alerts, theme);

    // Footer
    draw_footer(frame, main[6], ViewMode::All, theme);
}

fn draw_all_view_minimal(
    frame: &mut Frame,
    metrics: &Metrics,
    procs: &[ProcInfo],
    selected: usize,
    theme: Theme,
    sort_mode: SortMode,
) {
    let size = frame.size();

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(3),  // CPU & MEM Gauges only
            Constraint::Min(5),     // Processes
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    // Header
    draw_header(frame, main[0], ViewMode::All, theme, metrics.uptime);

    // CPU & MEM Gauges only (no graphs)
    let metrics_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main[1]);

    draw_cpu_gauge_minimal(frame, metrics_area[0], metrics, theme);
    draw_mem_gauge_minimal(frame, metrics_area[1], metrics, theme);

    // Processes
    draw_processes(frame, main[2], procs, selected, theme, sort_mode);

    // Footer
    draw_footer(frame, main[3], ViewMode::All, theme);
}

fn draw_cpu_gauge_minimal(frame: &mut Frame, area: Rect, metrics: &Metrics, theme: Theme) {
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

    frame.render_widget(gauge, area);
}

fn draw_mem_gauge_minimal(frame: &mut Frame, area: Rect, metrics: &Metrics, theme: Theme) {
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
                .title(format!("MEM {}% ({}/{} MB)", percent, used, total))
                .borders(Borders::ALL)
                .border_style(get_border_style(theme)),
        )
        .gauge_style(Style::default().fg(color))
        .percent(percent);

    frame.render_widget(gauge, area);
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

    draw_header(frame, main[0], ViewMode::Cpu, theme, metrics.uptime);
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

    draw_header(frame, main[0], ViewMode::Memory, theme, metrics.uptime);
    draw_memory_fullscreen(frame, main[1], metrics, theme, mem_history);
    draw_footer(frame, main[2], ViewMode::Memory, theme);
}

fn draw_process_view(frame: &mut Frame, procs: &[ProcInfo], selected: usize, theme: Theme, sort_mode: SortMode) {
    let size = frame.size();

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Processes
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    draw_header(frame, main[0], ViewMode::Process, theme, 0);
    draw_processes(frame, main[1], procs, selected, theme, sort_mode);
    draw_footer(frame, main[2], ViewMode::Process, theme);
}

fn draw_header(frame: &mut Frame, area: Rect, mode: ViewMode, theme: Theme, uptime_secs: u64) {
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

    // Capitalize OS name (e.g., "Windows" instead of "windows x86_64")
    let os_name = std::env::consts::OS;
    let os_info = format!(
        "{}{}",
        os_name.chars().next().unwrap().to_uppercase(),
        &os_name[1..]
    );

    // Format uptime as hours:minutes
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    let uptime_str = format!("{}h {:02}m", hours, minutes);

    // Get current local time
    let now = chrono::Local::now();
    let time_str = now.format("%H:%M:%S").to_string();

    // Split header into left and right sections for better visibility
    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),  // Left: App info + uptime
            Constraint::Percentage(30),  // Right: current time
        ])
        .split(area);

    // Left section: App name, OS, mode, uptime with clear labels
    let left_text = format!("SysOracle  |  OS: {}  |  View: {}  |  Uptime: {}", 
        os_info, mode_str, uptime_str);
    let left_header = Paragraph::new(left_text)
        .style(Style::default().bg(bg_color).fg(fg_color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Left);

    // Right section: Current time
    let right_text = format!("Time: {}", time_str);
    let right_header = Paragraph::new(right_text)
        .style(Style::default().bg(bg_color).fg(fg_color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Right);

    frame.render_widget(left_header, header_layout[0]);
    frame.render_widget(right_header, header_layout[1]);
}

fn draw_cpu_gauge_with_graph(
    frame: &mut Frame,
    area: Rect,
    metrics: &Metrics,
    theme: Theme,
    cpu_history: &[f32],
) {
    // Only render graph if area is large enough
    let can_render_graph = area.height >= 8;
    
    let split = if can_render_graph {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(5)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3)])
            .split(area)
    };

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
                .title(format!(" CPU: {:.1}% ", cpu))
                .borders(Borders::ALL)
                .border_style(get_border_style(theme)),
        )
        .gauge_style(Style::default().fg(color))
        .percent(cpu);

    frame.render_widget(gauge, split[0]);

    // Mini graph (only if space available)
    if can_render_graph && cpu_history.len() > 1 && split.len() > 1 {
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
    // Only render graph if area is large enough
    let can_render_graph = area.height >= 8;
    
    let split = if can_render_graph {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(5)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3)])
            .split(area)
    };

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
                .title(format!(" MEM: {}% ({} / {} MB) ", percent, used, total))
                .borders(Borders::ALL)
                .border_style(get_border_style(theme)),
        )
        .gauge_style(Style::default().fg(color))
        .percent(percent);

    frame.render_widget(gauge, split[0]);

    // Mini graph (only if space available)
    if can_render_graph && mem_history.len() > 1 && split.len() > 1 {
        draw_mini_graph(frame, split[1], mem_history, color, theme);
    }
}

fn draw_mini_graph(frame: &mut Frame, area: Rect, history: &[f32], color: Color, _theme: Theme) {
    // Safety check: don't render if area is too small
    if area.width < 10 || area.height < 3 {
        return;
    }

    // Clamp history length to available width to prevent graph clipping
    let max_points = (area.width.saturating_sub(4)) as usize; // Account for borders/padding
    let history_slice = if history.len() > max_points {
        &history[history.len() - max_points..]
    } else {
        history
    };

    if history_slice.is_empty() {
        return;
    }

    let data: Vec<(f64, f64)> = history_slice
        .iter()
        .enumerate()
        .map(|(i, &val)| (i as f64, val as f64))
        .collect();

    let dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(color))
        .data(&data);

    let x_max = history_slice.len().max(2) as f64;

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
    // Safety check: don't render if area is too small
    if area.width < 20 || area.height < 15 {
        // Render simple message
        let msg = Paragraph::new("Terminal too small for CPU view\nResize or switch to All view (press 'a')")
            .block(Block::default()
                .title("CPU View")
                .borders(Borders::ALL))
            .alignment(Alignment::Center);
        frame.render_widget(msg, area);
        return;
    }

    // Split area into graph and per-core bars
    let num_cores = metrics.cpu_cores.len();
    let cores_height = (num_cores as u16).min(12) + 2; // +2 for borders
    
    let split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),           // Graph
            Constraint::Length(cores_height), // Per-core bars
        ])
        .split(area);

    // Draw main CPU graph
    let cpu = metrics.cpu.clamp(0.0, 100.0);
    let color = if cpu > 80.0 {
        Color::Red
    } else if cpu > 60.0 {
        Color::Yellow
    } else {
        Color::Green
    };

    if cpu_history.len() > 1 && split[0].height >= 5 {
        // Clamp history to available width
        let max_points = (split[0].width.saturating_sub(10)) as usize;
        let history_slice = if cpu_history.len() > max_points {
            &cpu_history[cpu_history.len() - max_points..]
        } else {
            cpu_history
        };

        let data: Vec<(f64, f64)> = history_slice
            .iter()
            .enumerate()
            .map(|(i, &val)| (i as f64, val as f64))
            .collect();

        let dataset = Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(color))
            .data(&data);

        let x_max = history_slice.len().max(2) as f64;

        let chart = Chart::new(vec![dataset])
            .block(
                Block::default()
                    .title(format!("CPU Total: {:.1}%", cpu))
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

        frame.render_widget(chart, split[0]);
    }

    // Draw per-core CPU bars
    draw_cpu_cores(frame, split[1], &metrics.cpu_cores, theme);
}

fn draw_cpu_cores(
    frame: &mut Frame,
    area: Rect,
    cores: &[crate::metrics::CoreUsage],
    theme: Theme,
) {
    use ratatui::text::{Line, Span};
    
    // Safety check: don't render if area is too small
    if area.width < 30 || area.height < 3 {
        return;
    }
    
    let mut lines: Vec<Line> = Vec::new();
    
    // Calculate dynamic bar width based on terminal width
    // Format: " Core 0   ████████░░  80.0%"
    // Fixed parts: space(1) + name(8) + spaces(3) + percent(6) = 18 chars
    // Borders: 2 chars
    // Available for bar: width - 20
    let available_width = area.width.saturating_sub(20).max(10) as usize;
    let bar_width = available_width.min(40); // cap at 40 for very wide terminals
    
    for core in cores {
        let usage = core.usage.clamp(0.0, 100.0);
        let filled = ((usage / 100.0) * bar_width as f32) as usize;
        let empty = bar_width.saturating_sub(filled);
        
        let color = if usage > 80.0 {
            Color::Red
        } else if usage > 50.0 {
            Color::Yellow
        } else {
            Color::Green
        };
        
        let bar = format!(
            "{:<8} {}{}  {:.0}%",
            core.name,
            "█".repeat(filled),
            "░".repeat(empty),
            usage
        );
        
        lines.push(Line::from(vec![
            Span::raw(" "),
            Span::styled(bar, Style::default().fg(color)),
        ]));
    }
    
    frame.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title("Per-Core Usage")
                    .borders(Borders::ALL)
                    .border_style(get_border_style(theme)),
            ),
        area,
    );
}

fn draw_memory_fullscreen(
    frame: &mut Frame,
    area: Rect,
    metrics: &Metrics,
    theme: Theme,
    mem_history: &[f32],
) {
    // Safety check: don't render if area is too small
    if area.width < 20 || area.height < 10 {
        let msg = Paragraph::new("Terminal too small for Memory view\nResize or switch to All view (press 'a')")
            .block(Block::default()
                .title("Memory View")
                .borders(Borders::ALL))
            .alignment(Alignment::Center);
        frame.render_widget(msg, area);
        return;
    }

    let mem_percent = ((metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0) as f32;
    let color = if mem_percent > 85.0 {
        Color::Red
    } else if mem_percent > 65.0 {
        Color::Yellow
    } else {
        Color::Blue
    };

    if mem_history.len() > 1 && area.height >= 5 {
        // Clamp history to available width
        let max_points = (area.width.saturating_sub(10)) as usize;
        let history_slice = if mem_history.len() > max_points {
            &mem_history[mem_history.len() - max_points..]
        } else {
            mem_history
        };

        let data: Vec<(f64, f64)> = history_slice
            .iter()
            .enumerate()
            .map(|(i, &val)| (i as f64, val as f64))
            .collect();

        let dataset = Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(color))
            .data(&data);

        let x_max = history_slice.len().max(2) as f64;

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
        "↓ RX: {:.2} MB/s  (Total: {:.1} MB)\n↑ TX: {:.2} MB/s  (Total: {:.1} MB)",
        metrics.network_rx_speed, rx_mb,
        metrics.network_tx_speed, tx_mb
    );

    frame.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .title(" Network ")
                    .borders(Borders::ALL)
                    .border_style(get_border_style(theme)),
            )
            .style(Style::default().fg(Color::Green)),
        area,
    );
}

fn draw_gpu(frame: &mut Frame, area: Rect, metrics: &Metrics, theme: Theme) {
    let text = if let Some(ref gpu) = metrics.gpu {
        let mem_used_mb = gpu.memory_used as f64 / 1024.0 / 1024.0;
        let mem_total_mb = gpu.memory_total as f64 / 1024.0 / 1024.0;
        let mem_percent = gpu.memory_used as f64 / gpu.memory_total as f64 * 100.0;
        
        format!(
            "{}\nUsage: {:.1}%\nVRAM: {:.0} / {:.0} MB ({:.1}%)",
            gpu.name,
            gpu.usage,
            mem_used_mb,
            mem_total_mb,
            mem_percent
        )
    } else {
        "No NVIDIA GPU detected or feature disabled".to_string()
    };

    let color = if metrics.gpu.is_some() {
        Color::Magenta
    } else {
        Color::Gray
    };

    frame.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .title(" GPU ")
                    .borders(Borders::ALL)
                    .border_style(get_border_style(theme)),
            )
            .style(Style::default().fg(color)),
        area,
    );
}

fn draw_disks(frame: &mut Frame, area: Rect, metrics: &Metrics, theme: Theme) {
    use ratatui::text::{Line, Span};
    
    // Safety check: don't render if area is too small
    if area.width < 30 || area.height < 3 {
        return;
    }
    
    let mut lines: Vec<Line> = Vec::new();
    
    if metrics.disks.is_empty() {
        lines.push(Line::from(" No disks detected"));
    } else {
        // Calculate dynamic bar width based on available space
        let available_width = area.width.saturating_sub(40).max(10) as usize;
        let bar_width = available_width.min(20);
        
        for disk in &metrics.disks {
            let used_gb = disk.used as f64 / 1024.0 / 1024.0 / 1024.0;
            let total_gb = disk.total as f64 / 1024.0 / 1024.0 / 1024.0;
            let percent = (disk.used as f64 / disk.total as f64 * 100.0) as u16;
            
            let color = if percent > 90 {
                Color::Red
            } else if percent > 75 {
                Color::Yellow
            } else {
                Color::Cyan
            };
            
            let filled = ((percent as f32 / 100.0) * bar_width as f32) as usize;
            let empty = bar_width.saturating_sub(filled);
            
            let line = format!(
                " {:<12} {}{}  {}% ({:.1}/{:.1} GB)",
                disk.mount_point,
                "█".repeat(filled),
                "░".repeat(empty),
                percent,
                used_gb,
                total_gb
            );
            
            lines.push(Line::from(vec![
                Span::styled(line, Style::default().fg(color)),
            ]));
        }
    }
    
    frame.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" Disk Usage ")
                    .borders(Borders::ALL)
                    .border_style(get_border_style(theme)),
            ),
        area,
    );
}

fn draw_processes(
    frame: &mut Frame,
    area: Rect,
    procs: &[ProcInfo],
    selected: usize,
    theme: Theme,
    sort_mode: SortMode,
) {
    let (fg_color, selected_bg) = match theme {
        Theme::Dark => (Color::White, Color::Rgb(60, 60, 60)),
        Theme::Light => (Color::Black, Color::Rgb(200, 200, 200)),
    };

    let sort_indicator = match sort_mode {
        SortMode::Cpu => "CPU",
        SortMode::Memory => "MEM",
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
            .title(format!(" Processes [Sort: {}] - Use ↑↓ to navigate, 's' to toggle sort, 'k' to kill ", sort_indicator))
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
                    .title(" Alerts ")
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
        ViewMode::All => "a: All | c: CPU | m: Memory | p: Process | s: Sort | t: Theme | q: Quit",
        ViewMode::Cpu => "a: All | c: CPU | m: Memory | p: Process | t: Theme | q: Quit",
        ViewMode::Memory => "a: All | c: CPU | m: Memory | p: Process | t: Theme | q: Quit",
        ViewMode::Process => "a: All | ↑↓: Navigate | s: Sort | k: Kill | t: Theme | q: Quit",
    };

    let footer = Paragraph::new(keys)
        .style(Style::default().bg(bg_color).fg(fg_color))
        .alignment(Alignment::Center);

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