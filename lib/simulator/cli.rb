require 'io/console'

class CLI
  def initialize
    @running = true
    @commands = {
      's' => :change_servers,
      'r' => :change_arrival_rate,
      'p' => :change_processing_time,
      'q' => :quit,
      'stats' => :show_stats,
      'reset' => :reset_simulation
    }
  end

  def running?
    @running
  end

  def check_input
    return nil unless STDIN.ready?
    
    input = STDIN.gets&.chomp
    return nil if input.nil? || input.empty?

    parts = input.split(' ', 2)
    command = parts[0].downcase
    value = parts[1]

    case command
    when 's'
      { type: :servers, value: value&.to_i || 3 }
    when 'r'
      { type: :arrival_rate, value: value&.to_f || 2.0 }
    when 'p'
      { type: :processing_time, value: value&.to_f || 1.0 }
    when 'q'
      @running = false
      { type: :quit }
    when 'stats'
      { type: :show_stats }
    when 'reset'
      { type: :reset }
    else
      nil
    end
  end

  def stop
    @running = false
  end
end
