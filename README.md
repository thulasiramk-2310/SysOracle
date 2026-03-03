# SysOracle 🔮

**SysOracle** is a modern, cross-platform terminal system monitor with real-time graphs, Lua-based alerts, and a beautiful terminal UI. Built in **Rust** for performance and reliability.

![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-blue)
![Language](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## ✨ Features

* 🚀 **High-performance Rust core** - Minimal CPU/memory footprint
* 📊 **Real-time graphs** - Smooth scrolling CPU & memory history charts
* 🎨 **Modern TUI** - Clean, btop/htop inspired interface
* 🌗 **Theme support** - Dark and light themes (toggle with `t`)
* 🔄 **Multiple view modes** - Dashboard, CPU-only, Memory-only, Process-only
* 📝 **Process management** - Browse, sort, and kill processes with confirmation
* 🤖 **Lua scripting** - Programmable alerts and automation
* ⚡ **Configurable** - Adjust refresh rate, history size, and more
* 🌐 **Network stats** - Real-time speed (MB/s) and total bandwidth
* 🎯 **Keyboard-driven** - No mouse required
* 🔥 **Per-core CPU monitoring** - Individual core usage bars with color coding *(v2.0)*
* 💾 **Sort toggle** - Switch between CPU/Memory sorting *(v2.0)*
* ⚡ **Live network speed** - Real-time MB/s transfer rates *(v2.0)*
* 🎮 **GPU monitoring** - NVIDIA GPU usage and VRAM tracking *(v3.0)*
* 💿 **Disk usage** - Multi-disk space monitoring with visual bars *(v3.0)*
* ⏱️ **System uptime** - Live uptime display in header *(v3.0)*
* 📱 **Responsive layouts** - Auto-adapts to terminal size (FULL/COMPACT/MINIMAL) *(v3.0)*

## 🎮 Quick Start

### Installation

#### Prerequisites
* Rust (>= 1.75)
* Cargo
* (Optional) NVIDIA drivers for GPU monitoring

#### Build & Run

**Standard build:**
```bash
git clone https://github.com/your-username/sysoracle.git
cd sysoracle
cargo build --release
./target/release/sysoracle
```

**With GPU support:**
```bash
cargo build --release --features gpu
```

Or run directly in development mode:

```bash
cargo run
```

## 🎹 Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `a` | Show all panels (dashboard view) |
| `c` | CPU-only fullscreen view (with per-core bars) |
| `m` | Memory-only fullscreen view |
| `p` | Process-only fullscreen view |
| `s` | Toggle sort mode (CPU ⇄ Memory) |
| `t` | Toggle theme (Dark/Light) |
| `↑` / `↓` | Navigate process list |
| `k` | Kill selected process (with confirmation) |
| `r` | Refresh/reload |
| `q` | Quit application |

## 🖼️ UI Layout

### Dashboard View (Mode: ALL)

```
┌─────────────────────────────────────────────────────────────────┐
│  SysOracle v0.1.0  |  windows x86_64  |  Mode: ALL              │
├─────────────────────────────────────────────────────────────────┤
│ ┌──────────── CPU 45.2% ────────┐ ┌─────── MEM 68% ──────────┐ │
│ │ ████████████░░░░░░░░░░░░░░░░░ │ │ ████████████████░░░░░░░░ │ │
│ │                                │ │ 8192 / 12000 MB          │ │
│ │ ▁▂▃▄▅▄▃▂▁▂▃▄▅▆▅▄▃▂▁▂▃▄▅▄▃▂   │ │ ▂▃▄▅▄▃▂▁▂▃▄▅▄▃▂▁▂▃▄▅▄▃   │ │
│ └────────────────────────────────┘ └───────────────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────── Network ──────────────────────────────────┐ │
│ │  RX  ↓  1234.56 MB total                                     │ │
│ │  TX  ↑  456.78 MB total                                      │ │
│ └───────────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│ ┌────────────── Processes (↑↓ navigate | k kill) ─────────────┐ │
│ │ PID     NAME                    CPU%    MEM%                 │ │
│ │ 1234    chrome                  15.2    8.4                  │ │
│ │ 5678    vscode                  12.1    12.3                 │ │
│ │ 9012    rust-analyzer           8.5     4.2                  │ │
│ └───────────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────── Alerts ────────────────────────────────┐ │
│ │  ✓ No active alerts                                          │ │
│ └───────────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│  a All | c CPU | m Memory | p Process | t Theme | q Quit        │
└─────────────────────────────────────────────────────────────────┘
```

### CPU-Only View

Full-screen CPU usage graph with detailed history and statistics.

### Memory-Only View

Full-screen memory usage graph showing real-time consumption patterns.

### Process-Only View

Expanded process table with sorting and management capabilities.

## 📝 Configuration

### config.toml

Create or edit `config.toml` in the project root:

```toml
# SysOracle Configuration File

[general]
# Refresh rate in milliseconds (default: 800)
refresh_rate = 800

[lua]
# Directory containing Lua alert rules
rules_dir = "lua/rules"

[ui]
# Show process list in dashboard
show_processes = true

# History size for CPU/Memory graphs (default: 60)
history_size = 60
```

## 🤖 Lua Scripting

SysOracle supports custom Lua scripts for programmable alerts and automation.

### Available Variables

```lua
cpu.usage        -- Current CPU usage percentage (0-100)
mem.used         -- Memory used in bytes
mem.total        -- Total memory in bytes
mem.used_percent -- Memory usage percentage (0-100)
```

### Available Functions

```lua
notify(message)  -- Display an alert message
run(command)     -- Execute a shell command
```

### Example Rules

#### CPU Alert (`lua/rules/cpu.lua`)

```lua
if cpu.usage > 80 then
  notify("⚠ CPU usage above 80%")
  run("notify-send 'SysOracle: High CPU usage'")
end
```

#### Memory Alert (`lua/rules/memory.lua`)

```lua
if mem.used_percent > 85 then
  notify("⚠ Memory usage critical")
  run("notify-send 'SysOracle: High memory usage'")
end
```

#### Advanced Rule

```lua
-- Alert if CPU is high for extended period
local high_cpu_count = high_cpu_count or 0

if cpu.usage > 75 then
  high_cpu_count = high_cpu_count + 1
  if high_cpu_count > 10 then
    notify("⚠ Sustained high CPU usage detected")
    run("echo 'High CPU' >> /tmp/sysoracle.log")
  end
else
  high_cpu_count = 0
end
```

Rules are automatically loaded from `lua/rules/` directory.

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│          Terminal UI (ratatui)          │
│  Dashboard • Graphs • Tables • Themes   │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│            Rust Core                    │
│  App Logic • Metrics • Event Handling   │
└─────────┬───────────────┬───────────────┘
          │               │
┌─────────▼─────┐   ┌────▼──────────────┐
│  Lua Engine   │   │  Alert Engine     │
│  Rules • Auto │   │  Thresholds       │
└───────────────┘   └───────────────────┘
          │               │
┌─────────▼───────────────▼───────────────┐
│         System Metrics (sysinfo)        │
│  CPU • Memory • Network • Processes     │
└─────────────────────────────────────────┘
```

## 🛠️ Tech Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust 2021 |
| TUI Framework | ratatui + crossterm |
| System Metrics | sysinfo |
| Scripting | Lua 5.4 (mlua) |
| CLI | clap |
| Config | TOML |
| Async Runtime | tokio |

## 📂 Project Structure

```
sysoracle/
├── src/
│   ├── main.rs        # Entry point & initialization
│   ├── app.rs         # Application state & event loop
│   ├── tui.rs         # UI rendering & layouts
│   ├── metrics.rs     # System metrics collection
│   ├── alert.rs       # Alert engine
│   ├── lua_engine.rs  # Lua runtime & rule execution
│   ├── config.rs      # Configuration loading
│   └── cli.rs         # Command-line interface
│
├── lua/
│   └── rules/
│       ├── cpu.lua    # CPU monitoring rules
│       └── memory.lua # Memory monitoring rules
│
├── config.toml        # Configuration file
├── Cargo.toml         # Rust dependencies
└── README.md          # This file
```

## 🎯 Use Cases

* **Development monitoring** - Watch system resources while coding
* **Server monitoring** - Lightweight alternative to htop/btop
* **Learning Rust** - Well-structured example of TUI development
* **Scriptable alerts** - Custom notifications and automation
* **Cross-platform** - Works on Linux, macOS, and Windows

## 🚀 Roadmap

- [x] Real-time CPU & memory monitoring
- [x] Process list with sorting
- [x] Lua-based alert rules
- [x] Multiple view modes
- [x] Theme support
- [x] Scrolling history graphs
- [x] Network statistics
- [x] Kill process with confirmation
- [x] Configuration file support
- [ ] Disk I/O monitoring
- [ ] GPU usage tracking (NVIDIA/AMD)
- [ ] Export metrics to CSV/JSON
- [ ] Remote monitoring via HTTP API
- [ ] Plugin system
- [ ] More Lua API functions

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

* 🐛 Report bugs and issues
* 💡 Suggest new features
* 🔧 Submit pull requests
* 📚 Improve documentation
* ✨ Add new Lua rules
* 🎨 Enhance UI design

### Development Setup

```bash
# Clone the repository
git clone https://github.com/your-username/sysoracle.git
cd sysoracle

# Run in development mode
cargo run

# Run tests
cargo test

# Build release version
cargo build --release
```

## 📄 License

MIT License - see LICENSE file for details

## 👥 Authors

### **Ravindran S**
- Linux Systems
- Rust Development
- Full Stack Development
- [GitHub](https://github.com/ravindran-dev)

### **Thulasiram K**
- Full Stack Development
- Python
- Linux Systems
- [GitHub](https://github.com/thulasiramk-2310)

## 🙏 Acknowledgments

* [ratatui](https://github.com/ratatui-org/ratatui) - Amazing TUI framework
* [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - Cross-platform system information
* [mlua](https://github.com/khvzak/mlua) - High-level Lua bindings
* btop and htop for UI inspiration

---

**Made with ❤️ and Rust**

