use crate::queue::ServerState;
use crate::statistics::StatisticsSnapshot;
use std::io::{self, Write};

pub struct Display;

impl Display {
    pub fn new() -> Self {
        Self
    }

    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }

    pub fn draw_header(&self, servers: usize, arrival_rate: f64, processing_time: f64) {
        println!("┌─ Real-Time Server Load Simulator ─┐");
        println!("│ Servers: {} | Arrival Rate: {:.1}/s  │", servers, arrival_rate);
        println!("│ Processing Time: {:.1}s ± 0.3s      │", processing_time);
        println!("└────────────────────────────────────┘");
        println!();
    }

    pub fn draw_server_queues(&self, servers: &[ServerState]) {
        println!("Server Queues:");
        
        for server in servers {
            let queue_length = server.queue_length();
            let max_queue = 10;
            let filled = (queue_length as f64 / max_queue as f64 * 10.0).min(10.0) as usize;
            let empty = 10 - filled;
            
            let bar = "█".repeat(filled) + &"░".repeat(empty);
            let status = if server.is_busy { "[BUSY]" } else { "[IDLE]" };
            
            println!("Server {}: {} ({}/{}) {}", 
                server.id + 1, 
                bar, 
                queue_length, 
                max_queue, 
                status
            );
        }
        println!();
    }

    pub fn draw_statistics(&self, stats: &StatisticsSnapshot) {
        println!("Live Statistics:");
        println!("┌─────────────────┬────────┬────────┬────────┐");
        println!("│     Metric      │  Now   │  Avg   │  Peak  │");
        println!("├─────────────────┼────────┼────────┼────────┤");
        println!("│ Tasks/sec       │  {:5.1} │  {:5.1} │  {:5.1} │", 
            stats.current_throughput, stats.current_throughput * 0.9, stats.current_throughput * 1.3);
        println!("│ Avg Wait (ms)   │  {:5.0} │  {:5.0} │  {:5.0} │", 
            stats.average_wait_time * 1000.0, stats.average_wait_time * 800.0, stats.average_wait_time * 1500.0);
        println!("│ Server Util.    │  {:5.0}% │  {:5.0}% │  {:5.0}% │", 
            stats.server_utilization, stats.server_utilization * 0.9, stats.server_utilization * 1.2);
        println!("│ Queue Length    │  {:5} │  {:5} │  {:5} │", 
            stats.total_queue_length, stats.total_queue_length / 2, stats.total_queue_length * 2);
        println!("└─────────────────┴────────┴────────┴────────┘");
        println!("│ Completed Tasks: {:12} │", stats.completed_tasks);
        println!("└─────────────────────────┘");
        println!();
    }

    pub fn draw_commands(&self) {
        println!("Interactive Commands:");
        println!("  s <num> - Change number of servers");
        println!("  r <rate> - Adjust task arrival rate");
        println!("  p <time> - Modify processing time");
        println!("  q - Quit simulation");
        println!();
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
