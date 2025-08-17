require 'colorize'
require 'tty-cursor'
require 'tty-screen'

class Display
  def initialize
    @cursor = TTY::Cursor
    @screen_width = TTY::Screen.width
    @screen_height = TTY::Screen.height
  end

  def clear_screen
    print @cursor.clear_screen
    print @cursor.move_to(0, 0)
  end

  def draw_header(servers_count, arrival_rate, processing_time)
    puts "┌─ Real-Time Server Load Simulator ─┐".colorize(:cyan)
    puts "│ Servers: #{servers_count} | Arrival Rate: #{arrival_rate.round(1)}/s  │".colorize(:cyan)
    puts "│ Processing Time: #{processing_time.round(1)}s ± 0.2s      │".colorize(:cyan)
    puts "└────────────────────────────────────┘".colorize(:cyan)
    puts
  end

  def draw_server_queues(servers)
    puts "Server Queues:".colorize(:yellow)
    servers.each_with_index do |server, idx|
      queue_length = server[:queue_length] || 0
      is_busy = server[:is_busy] || false
      max_display = 10
      
      filled_bars = [queue_length, max_display].min
      empty_bars = max_display - filled_bars
      
      bar = "█" * filled_bars + "░" * empty_bars
      status = is_busy ? "[BUSY]".colorize(:red) : "[IDLE]".colorize(:green)
      
      puts "Server #{idx + 1}: #{bar} (#{queue_length}/#{max_display}) #{status}"
    end
    puts
  end

  def draw_statistics(stats)
    puts "Live Statistics:".colorize(:yellow)
    puts "┌─────────────────┬────────┬────────┬────────┐"
    puts "│     Metric      │  Now   │  Avg   │  Peak  │"
    puts "├─────────────────┼────────┼────────┼────────┤"
    
    throughput = stats[:current_throughput] || 0
    wait_time = ((stats[:average_wait_time] || 0) * 1000).round
    utilization = (stats[:server_utilization] || 0).round
    queue_length = stats[:total_queue_length] || 0
    
    puts "│ Tasks/sec       │  #{throughput.round(1).to_s.rjust(4)}  │  #{throughput.round(1).to_s.rjust(4)}  │  #{throughput.round(1).to_s.rjust(4)}  │"
    puts "│ Avg Wait (ms)   │ #{wait_time.to_s.rjust(5)}  │ #{wait_time.to_s.rjust(5)}  │ #{wait_time.to_s.rjust(5)}  │"
    puts "│ Server Util.    │  #{utilization}%   │  #{utilization}%   │  #{utilization}%   │"
    puts "│ Queue Length    │ #{queue_length.to_s.rjust(6)} │ #{queue_length.to_s.rjust(6)} │ #{queue_length.to_s.rjust(6)} │"
    puts "└─────────────────┴────────┴────────┴────────┘"
    puts
  end

  def draw_commands
    puts "Commands: [s]ervers [r]ate [p]rocessing [q]uit".colorize(:light_blue)
    print "> "
  end
end
