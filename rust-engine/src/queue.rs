use rand::Rng;
use std::collections::VecDeque;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub arrival_time: f64,
    pub processing_time: f64,
    pub start_time: f64,
    pub completion_time: f64,
}

#[repr(C)]
#[derive(Debug)]
pub struct ServerState {
    pub id: usize,
    pub queue: VecDeque<Task>,
    pub current_task: Option<Task>,
    pub is_busy: bool,
    pub total_processed: u64,
}

impl ServerState {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            queue: VecDeque::new(),
            current_task: None,
            is_busy: false,
            total_processed: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.queue.push_back(task);
    }

    pub fn process_step(&mut self, current_time: f64) -> Option<Task> {
        // Check if current task is complete
        if let Some(ref mut task) = self.current_task {
            if current_time >= task.start_time + task.processing_time {
                let mut completed_task = task.clone();
                completed_task.completion_time = current_time;
                self.current_task = None;
                self.is_busy = false;
                self.total_processed += 1;
                return Some(completed_task);
            }
        }

        // Start new task if idle and queue has tasks
        if !self.is_busy && !self.queue.is_empty() {
            if let Some(mut task) = self.queue.pop_front() {
                task.start_time = current_time;
                self.current_task = Some(task);
                self.is_busy = true;
            }
        }

        None
    }

    pub fn queue_length(&self) -> usize {
        self.queue.len()
    }
}

pub struct TaskGenerator {
    next_id: u64,
    arrival_rate: f64,
    processing_time: f64,
    processing_variance: f64,
}

impl TaskGenerator {
    pub fn new(arrival_rate: f64, processing_time: f64, processing_variance: f64) -> Self {
        Self {
            next_id: 1,
            arrival_rate,
            processing_time,
            processing_variance,
        }
    }

    pub fn generate_tasks(&mut self, delta_time: f64, current_time: f64) -> Vec<Task> {
        let mut rng = rand::thread_rng();
        let expected_tasks = self.arrival_rate * delta_time;
        let num_tasks = rng.gen::<f64>().powf(1.0 / expected_tasks).floor() as usize;

        (0..num_tasks)
            .map(|_| {
                let processing_time = (self.processing_time + 
                    rng.gen_range(-self.processing_variance..self.processing_variance))
                    .max(0.1);
                
                let task = Task {
                    id: self.next_id,
                    arrival_time: current_time,
                    processing_time,
                    start_time: 0.0,
                    completion_time: 0.0,
                };
                self.next_id += 1;
                task
            })
            .collect()
    }
}

pub fn select_least_loaded_server(servers: &[ServerState]) -> usize {
    servers
        .iter()
        .enumerate()
        .min_by_key(|(_, server)| server.queue_length())
        .map(|(idx, _)| idx)
        .unwrap_or(0)
}
