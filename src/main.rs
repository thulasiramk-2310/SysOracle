mod app;
mod metrics;
mod tui;
mod lua_engine;
mod alert;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    App::new()?.run()
}
