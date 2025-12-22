-- lua/cpu.lua
if cpu.usage > 80 then
  notify("CPU usage above 80%")
  run("notify-send 'SysOracle: High CPU usage'")
end
