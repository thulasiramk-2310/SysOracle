use std::{io, time::Duration};

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

pub struct App {
    system: System,
    lua: LuaEngine,
    selected_proc: usize,
    alerts: AlertEngine,
    view_mode: ViewMode,
    theme: Theme,
    cpu_history: Vec<f32>,
    mem_history: Vec<f32>,
    history_size: usize,
    show_kill_popup: bool,
    refresh_rate: u64,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let history_size = config.ui.history_size.unwrap_or(60);
        let refresh_rate = config.general.refresh_rate;
        
        Ok(Self {
            system: System::new_all(),
            lua: LuaEngine::new()?,
            selected_proc: 0,
            alerts: AlertEngine::new(),
            view_mode: ViewMode::All,
            theme: Theme::Dark,
            cpu_history: Vec::new(),
            mem_history: Vec::new(),
            history_size,
            show_kill_popup: false,
            refresh_rate,
        })
    }

    pub fn run(mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            // Collect metrics
            let metrics = Metrics::collect(&mut self.system);
            let processes: Vec<ProcInfo> = metrics::top_processes(&self.system, 20);

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
            let alert_list = self.alerts.list();

            // Execute Lua rules
            let _ = self.lua.execute(&metrics);

            // Draw UI
            terminal.draw(|f| {
                tui::draw(
                    f,
                    &metrics,
                    &processes,
                    &alert_list,
                    self.selected_proc,
                    self.view_mode,
                    self.theme,
                    &self.cpu_history,
                    &self.mem_history,
                    self.show_kill_popup,
                );
            })?;

            // Handle input
            if event::poll(Duration::from_millis(self.refresh_rate))? {
                if let Event::Key(key) = event::read()? {
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
                                // Reload/refresh
                                self.system.refresh_all();
                            }

                            _ => {}
                        }
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn kill_process(&mut self, pid: i32) {
        use sysinfo::Pid;
        let pid = Pid::from_u32(pid as u32);
        if let Some(process) = self.system.process(pid) {
            process.kill();
        }
    }
}
