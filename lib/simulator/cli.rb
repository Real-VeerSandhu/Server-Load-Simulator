require 'io/console'

class CLI
  def initialize
    @running = true
    @input_buffer = ""
  end

  def running?
    @running
  end

  def check_input
    # Check for available input without blocking
    if IO.select([STDIN], nil, nil, 0)
      char = STDIN.getch
      
      if char == "\r" || char == "\n"
        # Process the complete input
        input = @input_buffer.strip
        @input_buffer = ""
        
        return nil if input.empty?
        
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
      elsif char == "\u007F" || char == "\b" # Backspace
        @input_buffer = @input_buffer[0...-1] unless @input_buffer.empty?
        nil
      else
        # Add character to buffer
        @input_buffer += char
        nil
      end
    else
      nil
    end
  end

  def stop
    @running = false
  end
end
