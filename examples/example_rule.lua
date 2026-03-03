-- Example advanced Lua rule for SysOracle
-- This demonstrates the full capabilities of the scripting engine

-- CPU Monitoring
if cpu.usage > 90 then
  notify("🔥 CRITICAL: CPU usage at " .. string.format("%.1f", cpu.usage) .. "%")
elseif cpu.usage > 75 then
  notify("⚠️  WARNING: CPU usage at " .. string.format("%.1f", cpu.usage) .. "%")
end

-- Memory Monitoring
local mem_used_gb = mem.used / 1024 / 1024 / 1024
local mem_total_gb = mem.total / 1024 / 1024 / 1024

if mem.used_percent > 90 then
  notify(string.format("🔥 CRITICAL: Memory at %.1f%% (%.2f/%.2f GB)", 
    mem.used_percent, mem_used_gb, mem_total_gb))
elseif mem.used_percent > 80 then
  notify(string.format("⚠️  WARNING: Memory at %.1f%%", mem.used_percent))
end

-- Combined alert
if cpu.usage > 80 and mem.used_percent > 80 then
  notify("⚠️  System under heavy load!")
end
