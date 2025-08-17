require 'ffi'

module RustEngine
  extend FFI::Library
  
  # Try to load the library from the expected location
  lib_path = File.join(__dir__, '..', 'rust-engine', 'target', 'release', 'libqueue_engine.dylib')
  lib_path = File.join(__dir__, '..', 'rust-engine', 'target', 'release', 'libqueue_engine.so') unless File.exist?(lib_path)
  
  ffi_lib lib_path

  # Define C structs
  class ServerState < FFI::Struct
    layout :id, :size_t,
           :queue_length, :size_t,
           :is_busy, :bool,
           :total_processed, :uint64
  end

  class ProcessingResult < FFI::Struct
    layout :completed_tasks_count, :size_t,
           :new_tasks_count, :size_t
  end

  class StatisticsSnapshot < FFI::Struct
    layout :current_throughput, :double,
           :average_wait_time, :double,
           :server_utilization, :double,
           :total_queue_length, :size_t,
           :completed_tasks, :uint64
  end

  # Attach functions
  attach_function :init_engine, [:double, :double, :double], :void
  attach_function :process_simulation_step, [:pointer, :size_t, :double, :double], ProcessingResult.by_value
  attach_function :get_statistics, [:pointer, :size_t, :double], StatisticsSnapshot.by_value
  attach_function :create_server, [:size_t], :pointer
  attach_function :free_server, [:pointer], :void
  attach_function :update_arrival_rate, [:double], :void
end
