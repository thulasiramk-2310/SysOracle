use std::{io, time::Duration};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    execute,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use sysinfo::System;

use crate::{lua_engine::LuaEngine, metrics::Metrics, tui};

pub struct App {
    system: System,
    lua: LuaEngine,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            system: System::new_all(),
            lua: LuaEngine::new()?,
        })
    }

    pub fn run(mut self) -> Result<()> {
        // Terminal setup
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main loop
        loop {
            let metrics = Metrics::collect(&mut self.system);

            // Execute Lua rules
            let _ = self.lua.execute(&metrics);

            // Draw UI
            terminal.draw(|f| {
                tui::draw(f, &metrics);
            })?;

            // Handle input
            if event::poll(Duration::from_millis(1000))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }

        // Cleanup
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}
