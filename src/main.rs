mod app;
mod metrics;
mod tui;
mod lua_engine;
mod alert;
mod config;
mod cli;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    let config = config::load_config();
    App::new(config)?.run()
}
