#!/usr/bin/env ruby

require_relative 'lib/rust_engine'
require_relative 'lib/simulator/display'
require_relative 'lib/simulator/server'
require_relative 'lib/simulator/cli'

class Simulator
  def initialize(servers: 3, arrival_rate: 2.0, processing_time: 1.0)
    @servers_count = servers
    @arrival_rate = arrival_rate
    @processing_time = processing_time
    @processing_variance = 0.3
    
    @display = Display.new
    @cli = CLI.new
    @servers = []
    @server_pointers = []
    @start_time = Time.now
    @last_update = @start_time
    
    initialize_simulation
  end

  def initialize_simulation
    # Initialize Rust engine
    RustEngine.init_engine(@arrival_rate, @processing_time, @processing_variance)
    
    # Create servers
    cleanup_servers
    @servers = []
    @server_pointers = []
    
    @servers_count.times do |i|
      server = Server.new(i)
      server_ptr = RustEngine.create_server(i)
      @servers << server
      @server_pointers << server_ptr
    end
  end

  def cleanup_servers
    @server_pointers.each { |ptr| RustEngine.free_server(ptr) }
    @server_pointers.clear
  end

  def run
    puts "Starting Real-Time Queue & Server Load Simulator..."
    puts "Press Enter to begin, then use commands to control the simulation."
    gets

    begin
      main_loop
    rescue Interrupt
      puts "\nShutting down simulator..."
    ensure
      cleanup_servers
    end
  end

  private

  def main_loop
    while @cli.running?
      current_time = Time.now
      delta_time = current_time - @last_update
      @last_update = current_time
      
      # Process simulation step
      if !@server_pointers.empty?
        servers_array = FFI::MemoryPointer.new(RustEngine::ServerState, @servers_count)
        
        result = RustEngine.process_simulation_step(
          servers_array,
          @servers_count,
          delta_time,
          current_time - @start_time
        )
        
        # Update server states (simplified for prototype)
        @servers.each_with_index do |server, idx|
          server.instance_variable_set(:@queue_length, rand(0..10))
          server.instance_variable_set(:@is_busy, rand < 0.6)
          server.instance_variable_set(:@total_processed, server.total_processed + rand(0..2))
        end
      end

      # Get statistics
      stats = get_current_statistics
      
      # Update display
      @display.clear_screen
      @display.draw_header(@servers_count, @arrival_rate, @processing_time)
      @display.draw_server_queues(@servers.map(&:to_hash))
      @display.draw_statistics(stats)
      @display.draw_commands

      # Handle user input
      handle_input

      sleep(0.1) # 10 FPS update rate
    end
  end

  def get_current_statistics
    if !@server_pointers.empty?
      servers_array = FFI::MemoryPointer.new(RustEngine::ServerState, @servers_count)
      time_window = Time.now - @start_time
      
      rust_stats = RustEngine.get_statistics(servers_array, @servers_count, time_window)
      
      {
        current_throughput: rust_stats[:current_throughput],
        average_wait_time: rust_stats[:average_wait_time],
        server_utilization: rust_stats[:server_utilization],
        total_queue_length: rust_stats[:total_queue_length],
        completed_tasks: rust_stats[:completed_tasks]
      }
    else
      {
        current_throughput: 0.0,
        average_wait_time: 0.0,
        server_utilization: 0.0,
        total_queue_length: 0,
        completed_tasks: 0
      }
    end
  end

  def handle_input
    command = @cli.check_input
    return unless command

    case command[:type]
    when :servers
      @servers_count = [command[:value], 1].max
      initialize_simulation
      puts "Updated servers to #{@servers_count}"
    when :arrival_rate
      @arrival_rate = [command[:value], 0.1].max
      RustEngine.update_arrival_rate(@arrival_rate)
      puts "Updated arrival rate to #{@arrival_rate}"
    when :processing_time
      @processing_time = [command[:value], 0.1].max
      initialize_simulation # Reinitialize with new processing time
      puts "Updated processing time to #{@processing_time}"
    when :show_stats
      puts "Detailed statistics would be shown here"
    when :reset
      initialize_simulation
      puts "Simulation reset"
    end
  end
end

# Parse command line arguments
options = {
  servers: 3,
  arrival_rate: 2.0,
  processing_time: 1.0
}

ARGV.each_with_index do |arg, idx|
  case arg
  when '--servers'
    options[:servers] = ARGV[idx + 1].to_i if ARGV[idx + 1]
  when '--arrival-rate'
    options[:arrival_rate] = ARGV[idx + 1].to_f if ARGV[idx + 1]
  when '--processing-time'
    options[:processing_time] = ARGV[idx + 1].to_f if ARGV[idx + 1]
  end
end

# Run the simulator
simulator = Simulator.new(**options)
simulator.run
