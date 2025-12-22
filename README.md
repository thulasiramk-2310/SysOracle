# SysOracle

**SysOracle** is a fast, scriptable Linux system monitoring tool written in **Rust**, powered by **Lua** for extensibility and **shell hooks** for automation. It provides real-time observability inside a clean terminal UI (TUI), allowing users to define their own alert rules and system logic without recompiling the core application.



## Features

* **High-performance Rust core**
* **Lua-based rule engine** (script alerts & logic)
  **Terminal UI (TUI)** built with `ratatui`
* Real-time **CPU & Memory monitoring**
* Custom **alert rules** (threshold-based)
* Extensible architecture (rules, widgets, hooks)
* Linux-first design



## Architecture Overview

```
+-----------------------------+
|        Terminal UI          |
|        (ratatui)            |
+-------------+---------------+
              |
+-------------v---------------+
|        Rust Core             |
|  Metrics • App Loop • TUI    |
+-------------+---------------+
              |
+-------------v---------------+
|        Lua Engine            |
|  Rules • Alerts • Logic     |
+-------------+---------------+
              |
+-------------v---------------+
|        Linux Kernel          |
|     /proc • /sys             |
+-----------------------------+
```



## Tech Stack

| Layer          | Technology          |
| -------------- | ------------------- |
| Core           | Rust                |
| UI             | ratatui + crossterm |
| System Metrics | sysinfo             |
| Scripting      | Lua (mlua)          |
| Automation     | Shell scripts       |



## Project Structure

```
sysoracle/
├── src/
│   ├── main.rs        # Entry point
│   ├── app.rs         # App loop & terminal handling
│   ├── metrics.rs     # CPU & memory collection
│   ├── tui.rs         # UI rendering
│   └── lua_engine.rs  # Lua runtime & rule execution
│
├── lua/
│   └── rules/
│       ├── cpu.lua
│       └── memory.lua
│
├── Cargo.toml
└── README.md
```



## Installation

### Prerequisites

* Linux OS
* Rust (>= 1.75)
* Lua 5.4

### Clone & Run

```bash
git clone https://github.com/your-username/sysoracle.git
cd sysoracle
cargo run
```

Press **`q`** to quit the UI.



## Lua Rule Examples

### CPU Alert (`lua/rules/cpu.lua`)

```lua
if cpu.usage > 80 then
  notify("High CPU usage")
end
```

### Memory Alert (`lua/rules/memory.lua`)

```lua
if mem.used_percent > 85 then
  notify("High memory usage")
end
```

Rules are automatically loaded from the `lua/rules/` directory.


## Sample UI (Rust alone)

```
┌──────────────── SysOracle 🔮 ────────────────┐
│ Press Q to quit                               │
├──────────────────────────────────────────────┤
│ CPU Usage                                    │
│ ████████████████░░░░░░░░░░░  65%              │
├──────────────────────────────────────────────┤
│ Memory Usage                                 │
│ ██████████████████░░░░░░░░  72%               │
└──────────────────────────────────────────────┘
```



## Use Cases

* Developers monitoring local systems
* Linux enthusiasts & power users
* Learning **Rust system programming**
* Demonstrating **embedded Lua scripting**
* Hackathons & open-source contributions



## Roadmap

* [ ] Process list panel (htop-style)
* [ ] Alerts history UI
* [ ] Config file (`config.toml`)
* [ ] Hot-reload Lua rules
* [ ] systemd service support



##  Contributing

Contributions are welcome!

You can help by:

* Adding new Lua rules
* Improving UI layout
* Optimizing performance
* Writing documentation



##  License

MIT License



## Author
### **Ravindran S** 
- Linux
- Rust
- FSD
- [Reach me here](https://github.com/ravindran-dev)

   
### **Thulasiram K** 
- FSD
- Python
- Linux
- [Reach me here](https://github.com/thulasiramk-2310)


