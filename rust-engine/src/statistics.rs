use crate::queue::{Task, ServerState};

#[derive(Debug, Clone)]
pub struct StatisticsSnapshot {
    pub current_throughput: f64,
    pub average_wait_time: f64,
    pub server_utilization: f64,
    pub total_queue_length: usize,
    pub completed_tasks: u64,
}

impl Default for StatisticsSnapshot {
    fn default() -> Self {
        Self {
            current_throughput: 0.0,
            average_wait_time: 0.0,
            server_utilization: 0.0,
            total_queue_length: 0,
            completed_tasks: 0,
        }
    }
}

pub fn calculate_statistics(
    servers: &[ServerState],
    completed_tasks: &[Task],
    time_window: f64,
) -> StatisticsSnapshot {
    let total_queue_length = servers.iter().map(|s| s.queue_length()).sum();
    let busy_servers = servers.iter().filter(|s| s.is_busy).count();
    let server_utilization = if !servers.is_empty() {
        busy_servers as f64 / servers.len() as f64 * 100.0
    } else {
        0.0
    };

    let current_throughput = if time_window > 0.0 {
        completed_tasks.len() as f64 / time_window
    } else {
        0.0
    };

    let average_wait_time = if !completed_tasks.is_empty() {
        completed_tasks
            .iter()
            .map(|task| task.start_time - task.arrival_time)
            .sum::<f64>() / completed_tasks.len() as f64
    } else {
        0.0
    };

    let completed_tasks = servers.iter().map(|s| s.total_processed).sum();

    StatisticsSnapshot {
        current_throughput,
        average_wait_time,
        server_utilization,
        total_queue_length,
        completed_tasks,
    }
}
