# SysOracle 🔮

**SysOracle** is a modern, cross-platform terminal system monitor with real-time graphs, Lua-based alerts, and a beautiful terminal UI. Built in **Rust** for performance and reliability.

![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-blue)
![Language](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## ✨ Features

* **High-performance Rust core** - Minimal CPU/memory footprint
* **Real-time graphs** - Smooth scrolling CPU & memory history charts
* **Modern TUI** - Clean, btop/htop inspired interface
* **Theme support** - Dark and light themes (toggle with `t`)
* **Multiple view modes** - Dashboard, CPU-only, Memory-only, Process-only
* **Process management** - Browse, sort, and kill processes with confirmation
* **Lua scripting** - Programmable alerts and automation
* **Configurable** - Adjust refresh rate, history size, and more
* **Network stats** - Real-time speed (MB/s) and total bandwidth
* **Keyboard-driven** - No mouse required
* **Per-core CPU monitoring** - Individual core usage bars with color coding *(v2.0)*
* **Sort toggle** - Switch between CPU/Memory sorting *(v2.0)*
* **Live network speed** - Real-time MB/s transfer rates *(v2.0)*
* **GPU monitoring** - NVIDIA GPU usage and VRAM tracking *(v3.0)*
* **Disk usage** - Multi-disk space monitoring with visual bars *(v3.0)*
* **System uptime** - Live uptime display in header *(v3.0)*
* **Responsive layouts** - Auto-adapts to terminal size (FULL/COMPACT/MINIMAL) *(v3.0)*

## 🎮 Quick Start

### Installation

#### Prerequisites
* Rust (>= 1.75)
* Cargo
* (Optional) NVIDIA drivers for GPU monitoring

#### Build & Run

**Standard build:**
```bash
git clone https://github.com/thulasiramk-2310/sysoracle.git
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
┌───────────────────────────────────────────────────────────────────┐
│ SysOracle | OS: Windows | View: ALL | Uptime: 5h 23m | Time: 14:32:15 │
├───────────────────────────────────────────────────────────────────┤
│ ┌────────── CPU 45.2% ──────────┐ ┌────── MEM 68% ───────┐      │
│ │ ████████████░░░░░░░░░░░░░░░░  │ │ █████████████████░░░ │      │
│ │ 4 cores @ 3.6 GHz              │ │ 5.2 / 16 GB          │      │
│ │ ▁▂▃▄▅▄▃▂▁▂▃▄▅▆▅▄▃▂▁▂▃▄▅▄▃▂  │ │ ▂▃▄▅▄▃▂▁▂▃▄▅▄▃▂▁ │      │
│ └────────────────────────────────┘ └──────────────────────┘      │
├───────────────────────────────────────────────────────────────────┤
│ ┌────────────── Network ────────────┐ ┌───── GPU (NVIDIA) ─────┐ │
│ │ RX ↓ 15.2 MB/s (1234 MB total)    │ │ Usage: 45%            │ │
│ │ TX ↑ 3.4 MB/s (456 MB total)      │ │ VRAM: 2.1 / 8 GB      │ │
│ └────────────────────────────────────┘ └───────────────────────┘ │
├───────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────── Disks ────────────────────────────┐     │
│ │ C:\ ████████████████░░░░░░░░ 256 / 512 GB (50%)          │     │
│ │ D:\ ████████░░░░░░░░░░░░░░░░ 400 / 2000 GB (20%)         │     │
│ └──────────────────────────────────────────────────────────┘     │
├───────────────────────────────────────────────────────────────────┤
│ ┌──────────── Processes (↑↓ navigate | k kill) ───────────┐     │
│ │ PID     NAME                CPU%    MEM%                 │     │
│ │ 1234    chrome              15.2    8.4                  │     │
│ │ 5678    vscode              12.1    12.3                 │     │
│ │ 9012    rust-analyzer       8.5     4.2                  │     │
│ └──────────────────────────────────────────────────────────┘     │
├───────────────────────────────────────────────────────────────────┤
│ ┌──────────────────────── Alerts ──────────────────────────┐     │
│ │ ✓ No active alerts                                       │     │
│ └──────────────────────────────────────────────────────────┘     │
├───────────────────────────────────────────────────────────────────┤
│ a All | c CPU | m Memory | p Process | s Sort | t Theme | q Quit │
└───────────────────────────────────────────────────────────────────┘
```

### CPU-Only View (Press `c`)

Full-screen CPU usage graph with:
- Overall CPU percentage and history graph
- Per-core usage bars with individual percentages
- Color-coded bars (green/yellow/red based on load)
- Scrolling 60-point history

### Memory-Only View (Press `m`)

Full-screen memory usage showing:
- Used/total memory in GB and percentage
- Real-time consumption graph
- Color-coded gauge visualization
- 60-point scrolling history

### Process-Only View (Press `p`)

Expanded process table with:
- Full process list with PID, name, CPU%, and memory%
- Sortable by CPU or memory (toggle with `s`)
- Kill process capability (select and press `k`)
- Confirmation dialog for safety

### GPU Monitoring *(v3.0)*

When built with `--features gpu`, displays:
- NVIDIA GPU utilization percentage
- VRAM usage (used/total in GB)
- Real-time monitoring (requires NVIDIA drivers)
- Graceful fallback if GPU not detected

### Disk Usage *(v3.0)*

Shows all mounted disks with:
- Drive letters/mount points
- Visual usage bars (color-coded)
- Used/total space in GB
- Percentage utilization

### Responsive Layouts *(v3.0)*

Auto-adapts to terminal size:
- **FULL** (≥40 lines): All panels with graphs
- **COMPACT** (25-39 lines): Condensed layout
- **MINIMAL** (<25 lines): Essential info only

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
│  Timer-based Refresh • Selective Update │
└─────────┬───────────────┬───────────────┘
          │               │
┌─────────▼─────┐   ┌────▼──────────────┐
│  Lua Engine   │   │  Alert Engine     │
│  Rules • Auto │   │  Thresholds       │
└───────────────┘   └───────────────────┘
          │               │
┌─────────▼───────────────▼───────────────┐
│      System Metrics Collection          │
├─────────────────────────────────────────┤
│ sysinfo: CPU • Memory • Net • Process   │
│ nvml-wrapper: GPU (NVIDIA) • VRAM       │
│ Disks API: Multi-disk space monitoring  │
│ chrono: System time & uptime tracking   │
└─────────────────────────────────────────┘
```

## 🛠️ Tech Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust 2021 |
| TUI Framework | ratatui + crossterm |
| System Metrics | sysinfo |
| GPU Monitoring | nvml-wrapper (optional) |
| Time Handling | chrono |
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

## ⚡ Performance

SysOracle is highly optimized for minimal system overhead:

* **Idle CPU usage:** ~0.1-0.5% (90-95% reduction from earlier versions)
* **Memory footprint:** ~5-10 MB
* **Timer-based refresh:** Configurable intervals (default: 800ms)
* **Selective system refresh:** Only refreshes metrics needed for current view
* **Responsive input:** 50ms polling for instant keyboard response
* **Efficient rendering:** Smart redraw triggers, minimal screen updates

The event loop architecture decouples user input from metric collection, ensuring smooth navigation even during intensive system monitoring.

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
- [x] Disk usage monitoring *(v3.0)*
- [x] GPU usage tracking (NVIDIA) *(v3.0)*
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

# Run with GPU support
cargo run --features gpu

# Run tests
cargo test

# Build release version
cargo build --release

# Build with GPU monitoring
cargo build --release --features gpu
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
---

**Made with ❤️ and Rust**

