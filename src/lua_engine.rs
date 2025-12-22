use mlua::{Lua, Result as LuaResult};
use std::fs;
use std::process::Command;

use crate::metrics::Metrics;

pub struct LuaEngine {
    lua: Lua,
}

impl LuaEngine {
    /// Initialize Lua engine and register global functions
    pub fn new() -> LuaResult<Self> {
        let lua = Lua::new();

        // notify("message")
        lua.globals().set(
            "notify",
            lua.create_function(|_, msg: String| {
                println!("⚠ ALERT: {}", msg);
                Ok(())
            })?,
        )?;

        // run("shell command")
        lua.globals().set(
            "run",
            lua.create_function(|_, cmd: String| {
                let _ = Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .spawn();
                Ok(())
            })?,
        )?;

        Ok(Self { lua })
    }

    /// Execute all Lua rules with current metrics
    pub fn execute(&self, metrics: &Metrics) -> LuaResult<()> {
        let globals = self.lua.globals();

        // ---- expose CPU table ----
        let cpu = self.lua.create_table()?;
        cpu.set("usage", metrics.cpu)?;
        globals.set("cpu", cpu)?;

        // ---- expose Memory table ----
        let mem = self.lua.create_table()?;
        mem.set("used", metrics.memory_used)?;
        mem.set("total", metrics.memory_total)?;
        mem.set(
            "used_percent",
            (metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0,
        )?;
        globals.set("mem", mem)?;

        // ---- load and execute all Lua rules ----
        for script in load_lua_rules("lua/rules") {
            let _ = self.lua.load(&script).exec();
        }

        Ok(())
    }
}

/// Load all `.lua` files from a directory
fn load_lua_rules(dir: &str) -> Vec<String> {
    let mut scripts = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return scripts,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("lua") {
            if let Ok(content) = fs::read_to_string(&path) {
                scripts.push(content);
            }
        }
    }

    scripts
}
