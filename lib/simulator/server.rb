class Server
  attr_reader :id, :queue_length, :is_busy, :total_processed

  def initialize(id)
    @id = id
    @queue_length = 0
    @is_busy = false
    @total_processed = 0
  end

  def update_from_rust(rust_server_data)
    @queue_length = rust_server_data[:queue_length] || 0
    @is_busy = rust_server_data[:is_busy] || false
    @total_processed = rust_server_data[:total_processed] || 0
  end

  def to_hash
    {
      id: @id,
      queue_length: @queue_length,
      is_busy: @is_busy,
      total_processed: @total_processed
    }
  end
end
