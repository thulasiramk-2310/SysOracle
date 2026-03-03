use std::{io, time::{Duration, Instant}};

use crate::alert::AlertEngine;
use crate::config::Config;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    execute,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use sysinfo::System;

use crate::{
    lua_engine::LuaEngine,
    metrics::{self, Metrics, ProcInfo},
    tui,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    All,
    Cpu,
    Memory,
    Process,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortMode {
    Cpu,
    Memory,
}

pub struct App {
    system: System,
    lua: LuaEngine,
    selected_proc: usize,
    alerts: AlertEngine,
    view_mode: ViewMode,
    theme: Theme,
    sort_mode: SortMode,
    cpu_history: Vec<f32>,
    mem_history: Vec<f32>,
    history_size: usize,
    show_kill_popup: bool,
    refresh_rate: u64,
    prev_network_rx: u64,
    prev_network_tx: u64,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let history_size = config.ui.history_size.unwrap_or(60);
        let refresh_rate = config.general.refresh_rate;
        
        let mut system = System::new();
        // Initial CPU refresh with proper timing
        system.refresh_cpu();
        std::thread::sleep(std::time::Duration::from_millis(200));
        system.refresh_cpu();
        
        // Initial refresh of other components
        system.refresh_memory();
        system.refresh_processes();
        
        Ok(Self {
            system,
            lua: LuaEngine::new()?,
            selected_proc: 0,
            alerts: AlertEngine::new(),
            view_mode: ViewMode::All,
            theme: Theme::Dark,
            sort_mode: SortMode::Cpu,
            cpu_history: Vec::new(),
            mem_history: Vec::new(),
            history_size,
            show_kill_popup: false,
            refresh_rate,
            prev_network_rx: 0,
            prev_network_tx: 0,
        })
    }

    pub fn run(mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Timer for metric refresh
        let mut last_refresh = Instant::now();
        let refresh_interval = Duration::from_millis(self.refresh_rate);
        
        // Cached data
        let mut metrics = Metrics::collect(
            &mut self.system,
            self.prev_network_rx,
            self.prev_network_tx,
            self.refresh_rate,
        );
        self.prev_network_rx = metrics.network_rx;
        self.prev_network_tx = metrics.network_tx;
        
        let mut processes: Vec<ProcInfo> = metrics::top_processes(&self.system, 20);
        self.sort_processes(&mut processes);
        
        // Initialize history
        self.cpu_history.push(metrics.cpu);
        let mem_percent = (metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0;
        self.mem_history.push(mem_percent as f32);
        
        // Update alerts initially
        self.alerts.update(&metrics);
        let _ = self.lua.execute(&metrics);
        
        let mut needs_redraw = true;
        let mut sort_changed = false;

        loop {
            // Check if it's time to refresh metrics
            let now = Instant::now();
            let should_refresh = now.duration_since(last_refresh) >= refresh_interval;

            if should_refresh {
                // Selective refresh based on view mode
                self.selective_refresh();
                
                // Collect new metrics
                metrics = Metrics::collect(
                    &mut self.system,
                    self.prev_network_rx,
                    self.prev_network_tx,
                    self.refresh_rate,
                );
                
                // Update previous network values for next cycle
                self.prev_network_rx = metrics.network_rx;
                self.prev_network_tx = metrics.network_tx;

                // Get and sort processes
                processes = metrics::top_processes(&self.system, 20);
                self.sort_processes(&mut processes);

                // Clamp selected process index
                if self.selected_proc >= processes.len() && !processes.is_empty() {
                    self.selected_proc = processes.len() - 1;
                }

                // Update history
                self.cpu_history.push(metrics.cpu);
                if self.cpu_history.len() > self.history_size {
                    self.cpu_history.remove(0);
                }

                let mem_percent = (metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0;
                self.mem_history.push(mem_percent as f32);
                if self.mem_history.len() > self.history_size {
                    self.mem_history.remove(0);
                }

                // Update alerts
                self.alerts.update(&metrics);
                let _ = self.lua.execute(&metrics);

                last_refresh = now;
                needs_redraw = true;
            }

            // If sort mode changed, resort immediately
            if sort_changed {
                self.sort_processes(&mut processes);
                self.selected_proc = 0;
                sort_changed = false;
                needs_redraw = true;
            }

            // Draw UI only when needed
            if needs_redraw {
                let alert_list = self.alerts.list();
                terminal.draw(|f| {
                    tui::draw(
                        f,
                        &metrics,
                        &processes,
                        &alert_list,
                        self.selected_proc,
                        self.view_mode,
                        self.theme,
                        self.sort_mode,
                        &self.cpu_history,
                        &self.mem_history,
                        self.show_kill_popup,
                    );
                })?;
                needs_redraw = false;
            }

            // Handle input with short timeout for responsiveness
            let timeout = Duration::from_millis(50); // Responsive input
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    needs_redraw = true; // Redraw on any input
                    
                    if self.show_kill_popup {
                        match key.code {
                            KeyCode::Char('y') | KeyCode::Char('Y') => {
                                if let Some(proc) = processes.get(self.selected_proc) {
                                    self.kill_process(proc.pid);
                                }
                                self.show_kill_popup = false;
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                                self.show_kill_popup = false;
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') => break,

                            KeyCode::Char('a') => {
                                self.view_mode = ViewMode::All;
                            }

                            KeyCode::Char('c') => {
                                self.view_mode = ViewMode::Cpu;
                            }

                            KeyCode::Char('m') => {
                                self.view_mode = ViewMode::Memory;
                            }

                            KeyCode::Char('p') => {
                                self.view_mode = ViewMode::Process;
                            }

                            KeyCode::Char('t') => {
                                self.theme = match self.theme {
                                    Theme::Dark => Theme::Light,
                                    Theme::Light => Theme::Dark,
                                };
                            }

                            KeyCode::Char('s') => {
                                self.sort_mode = match self.sort_mode {
                                    SortMode::Cpu => SortMode::Memory,
                                    SortMode::Memory => SortMode::Cpu,
                                };
                                sort_changed = true;
                            }

                            KeyCode::Up => {
                                if self.selected_proc > 0 {
                                    self.selected_proc -= 1;
                                }
                            }

                            KeyCode::Down => {
                                if self.selected_proc + 1 < processes.len() {
                                    self.selected_proc += 1;
                                }
                            }

                            KeyCode::Char('k') => {
                                self.show_kill_popup = true;
                            }

                            KeyCode::Char('r') => {
                                // Force immediate refresh
                                self.selective_refresh();
                                last_refresh = Instant::now().checked_sub(refresh_interval).unwrap_or(Instant::now());
                            }

                            _ => {
                                needs_redraw = false; // Unknown key, don't redraw
                            }
                        }
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn selective_refresh(&mut self) {
        // Selective refresh based on view mode to reduce CPU usage
        match self.view_mode {
            ViewMode::All => {
                // Refresh everything
                self.system.refresh_cpu();
                self.system.refresh_memory();
                self.system.refresh_processes();
                #[cfg(feature = "gpu")]
                {
                    // GPU refresh handled in Metrics::collect
                }
            }
            ViewMode::Cpu => {
                // Only CPU needed
                self.system.refresh_cpu();
            }
            ViewMode::Memory => {
                // Only memory needed
                self.system.refresh_memory();
            }
            ViewMode::Process => {
                // CPU and processes for process view
                self.system.refresh_cpu();
                self.system.refresh_processes();
            }
        }
        // Networks are refreshed via Networks::new_with_refreshed_list() in Metrics::collect()
    }

    fn sort_processes(&self, processes: &mut Vec<ProcInfo>) {
        match self.sort_mode {
            SortMode::Cpu => {
                processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));
            }
            SortMode::Memory => {
                processes.sort_by(|a, b| b.mem.partial_cmp(&a.mem).unwrap_or(std::cmp::Ordering::Equal));
            }
        }
    }

    fn kill_process(&mut self, pid: i32) {
        use sysinfo::Pid;
        let pid = Pid::from_u32(pid as u32);
        if let Some(process) = self.system.process(pid) {
            process.kill();
        }
    }
}
