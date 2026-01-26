use std::{io, time::Duration};

use crate::alert::AlertEngine;

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

pub struct App {
    system: System,
    lua: LuaEngine,
    selected_proc: usize,
    alerts: AlertEngine,        
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            system: System::new_all(),
            lua: LuaEngine::new()?,
            selected_proc: 0,
            alerts: AlertEngine::new(),        
        })
    }

    pub fn run(mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            
            let metrics = Metrics::collect(&mut self.system);
            let processes: Vec<ProcInfo> = metrics::top_processes(&self.system, 20);

            
            self.alerts.update(&metrics);
            let alert_list = self.alerts.list();

            
            let _ = self.lua.execute(&metrics);

            
            terminal.draw(|f| {
                tui::draw(
                    f,
                    &metrics,
                    &processes,
                    &alert_list,        
                    self.selected_proc,
                );
            })?;

            
            if event::poll(Duration::from_millis(800))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,

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

                        KeyCode::Char('r') => {
                            
                        }

                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}
