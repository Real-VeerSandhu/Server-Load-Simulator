# Real-Time Queue & Server Load Simulator

A high-performance real-time simulation system that models server load balancing and queue management, built entirely in Rust for maximum performance and efficiency.

```
┌─ Real-Time Server Load Simulator ─┐
│ Servers: 4 | Arrival Rate: 2.1/s  │
│ Processing Time: 0.8s ± 0.2s      │
└────────────────────────────────────┘

Server Queues:
Server 1: ████████░░ (8/10) [BUSY]
Server 2: ██░░░░░░░░ (2/10) [IDLE] 
Server 3: ██████████ (10/10) [BUSY]
Server 4: █████░░░░░ (5/10) [BUSY]

Live Statistics:
┌─────────────────┬────────┬────────┬────────┐
│     Metric      │  Now   │  Avg   │  Peak  │
├─────────────────┼────────┼────────┼────────┤
│ Tasks/sec       │   2.3  │   2.1  │   3.4  │
│ Avg Wait (ms)   │  450   │  380   │  1200  │
│ Server Util.    │  75%   │  68%   │   95%  │
│ Queue Length    │   25   │   18   │   42   │
└─────────────────┴────────┴────────┴────────┘

> r 3.0  # Increase arrival rate to 3.0 tasks/sec
```

## Overview

This simulator creates a realistic environment where you can observe and control multiple servers processing incoming tasks in real-time. Watch as tasks queue up, servers get busy, and performance metrics change dynamically based on your configuration.

**Key Features:**

- Real-time visualization with ASCII charts and live statistics
- Dynamic control of simulation parameters while running
- High-performance task processing using pure Rust
- Interactive CLI for easy monitoring and control
- Realistic server load balancing algorithms

## Architecture

The project leverages Rust's performance and safety features:

### Core Components

- **Task Processing Engine**: High-speed queue operations and task lifecycle management
- **Statistics Calculator**: Real-time computation of metrics (latency, throughput, utilization)
- **Load Balancing Algorithms**: Efficient server selection and task distribution
- **Random Task Generator**: High-volume task creation with configurable patterns
- **Interactive Display**: ASCII-based real-time visualization
- **Command Interface**: Non-blocking input handling for dynamic control

## Prerequisites

- **Rust** 1.70+ with Cargo
- **System**: Linux, macOS, or Windows with WSL

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/queue-server-simulator.git
cd queue-server-simulator
```

### 2. Build the Simulator

```bash
cd rust-engine
cargo build --release
```

## Usage

### Basic Simulation

```bash
cd rust-engine
cargo run --bin simulator
```

### Advanced Configuration

```bash
cargo run --bin simulator -- --servers 5 --arrival-rate 2.5 --processing-time 1.0
```

### Interactive Commands (while running)

- `s <num>`: Change number of servers
- `r <rate>`: Adjust task arrival rate
- `p <time>`: Modify processing time
- `q`: Quit simulation

<<<<<<< HEAD
=======
## Sample Output

```text
┌─ Real-Time Server Load Simulator ─┐
│ Servers: 4 | Arrival Rate: 2.1/s  │
│ Processing Time: 0.8s ± 0.3s      │
└───────────────────────────────────┘

Server Queues:
Server 1: ████████░░ (8/10) [BUSY]
Server 2: ██░░░░░░░░ (2/10) [IDLE] 
Server 3: ██████████ (10/10) [BUSY]
Server 4: █████░░░░░ (5/10) [BUSY]

Live Statistics:
┌─────────────────┬────────┬────────┬────────┐
│     Metric      │  Now   │  Avg   │  Peak  │
├─────────────────┼────────┼────────┼────────┤
│ Tasks/sec       │   2.3  │   2.1  │   3.4  │
│ Avg Wait (ms)   │  450   │  380   │  1200  │
│ Server Util.    │  75%   │  68%   │   95%  │
│ Queue Length    │   25   │   18   │   42   │
└─────────────────┴────────┴────────┴────────┘
│ Completed Tasks:         1234              │
└────────────────────────────────────────────┘

Interactive Commands:
  s <num> - Change number of servers
  r <rate> - Adjust task arrival rate
  p <time> - Modify processing time
  q - Quit simulation

> 
```

>>>>>>> ea2e6af (v0.2)
## Configuration

### Command Line Arguments

```bash
cargo run --bin simulator -- [OPTIONS]

OPTIONS:
    --servers <NUM>           Number of processing servers [default: 3]
    --arrival-rate <RATE>    Tasks per second arrival rate [default: 2.0]
    --processing-time <TIME> Average task processing time in seconds [default: 1.0]
```

### Configuration File (`config.yml`)

```yaml
simulation:
  servers: 3
  arrival_rate: 2.0
  processing_time: 1.0
  processing_variance: 0.3

display:
  refresh_rate: 10  # Hz
  chart_width: 60
  show_histograms: true

simulator:
  binary_path: "./rust-engine/target/release/simulator"
  max_queue_size: 50
```

### Environment Variables

```bash
export RUST_LOG=info                    # Rust logging level
export SIMULATOR_REFRESH_MS=100         # Display refresh rate
export MAX_QUEUE_SIZE=50                # Per-server queue limit
```

## Project Structure

```text
queue-server-simulator/
├── README.md
├── config.yml
└── rust-engine/
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs              # Main application entry point
    │   ├── lib.rs               # Library exports
    │   ├── queue.rs             # Queue processing engine
    │   ├── statistics.rs        # Performance calculations  
    │   └── display.rs            # ASCII visualization
    └── target/                   # Build artifacts
```

## Testing

```bash
# Run Rust tests  
cd rust-engine && cargo test

# Run with debug logging
RUST_LOG=debug cargo run --bin simulator

# Performance benchmarks
cd rust-engine && cargo bench
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

### Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add development components
rustup component add clippy rustfmt

# Run development version
cargo run --bin simulator -- --servers 5 --arrival-rate 3.0
```
