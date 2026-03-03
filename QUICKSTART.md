# SysOracle - Quick Start Guide

## First Run

### 1. Build the Project
```powershell
cargo build --release
```

### 2. Run SysOracle
```powershell
.\target\release\sysoracle.exe
# or in development mode
cargo run
```

## Navigation

Once launched, you'll see the dashboard view with:
- CPU gauge and history graph
- Memory gauge and history graph  
- Network statistics
- Process list
- Alerts panel
- Footer with keyboard shortcuts

## Keyboard Controls

| Key | Description |
|-----|-------------|
| `a` | Show all panels (dashboard view) |
| `c` | Switch to CPU-only fullscreen view |
| `m` | Switch to Memory-only fullscreen view |
| `p` | Switch to Process-only fullscreen view |
| `t` | Toggle between Dark and Light themes |
| `↑`/`↓` | Navigate through the process list |
| `k` | Kill the selected process (shows confirmation popup) |
| `r` | Refresh/reload system information |
| `q` | Quit the application |

## Try These Features

### 1. View Different Modes
- Press `c` to see a full-screen CPU graph
- Press `m` to see a full-screen Memory graph
- Press `p` to see an expanded process list
- Press `a` to return to the dashboard

### 2. Change Themes
- Press `t` to toggle between dark and light themes
- The entire interface will instantly switch appearance

### 3. Manage Processes
- Use `↑` and `↓` to select a process
- Press `k` to kill the selected process
- Confirm with `Y` or cancel with `N`

### 4. Monitor Alerts
Watch the Alerts panel for warnings when:
- CPU usage exceeds 80%
- Memory usage exceeds 85%

## Customization

### Edit Config File
Modify `config.toml`:
```toml
[general]
refresh_rate = 800  # Change to 500 for faster updates

[ui]
history_size = 100  # Show more history in graphs
```

### Create Custom Lua Rules
Add files to `lua/rules/`:
```lua
-- disk_alert.lua
if cpu.usage > 70 and mem.used_percent > 70 then
  notify("⚠️ System load high!")
end
```

## Troubleshooting

### Application Not Starting
- Ensure you built with: `cargo build --release`
- Check that your terminal supports color

### Graphs Not Showing
- Wait a few seconds for history to accumulate
- Graphs need at least 2 data points to render

### Lua Alerts Not Working
- Verify files in `lua/rules/` have `.lua` extension
- Check Lua syntax is valid
- Alerts update every refresh cycle (800ms by default)

## Performance Tips

1. **Lower refresh rate** for slower updates (less CPU usage)
2. **Reduce history size** if using an older machine
3. **Disable alerts** by removing Lua rules if not needed

## Next Steps

- Create custom Lua alert rules
- Experiment with different themes and view modes
- Integrate with system notifications (see README)
- Monitor specific processes for your workflow

## Need Help?

- Check the full README.md for detailed documentation
- Review example Lua rules in `lua/rules/`
- See example_rule.lua for advanced scripting

---
**Happy Monitoring! 🚀**
