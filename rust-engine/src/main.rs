use std::io;
use std::thread;
use std::time::{Duration, Instant};

mod queue;
mod statistics;
mod display;

use queue::{ServerState, TaskGenerator, select_least_loaded_server};
use statistics::calculate_statistics;
use display::Display;

struct Simulator {
    servers: Vec<ServerState>,
    task_generator: TaskGenerator,
    display: Display,
    running: bool,
    start_time: Instant,
    last_update: Instant,
    completed_tasks: Vec<queue::Task>,
}

impl Simulator {
    fn new(servers: usize, arrival_rate: f64, processing_time: f64) -> Self {
        let servers = (0..servers).map(ServerState::new).collect();
        let task_generator = TaskGenerator::new(arrival_rate, processing_time, 0.3);
        let display = Display::new();
        
        Self {
            servers,
            task_generator,
            display,
            running: true,
            start_time: Instant::now(),
            last_update: Instant::now(),
            completed_tasks: Vec::new(),
        }
    }

    fn run(&mut self) {
        println!("Starting Real-Time Queue & Server Load Simulator...");
        println!("Press Enter to begin, then use commands to control the simulation.");
        let _ = io::stdin().read_line(&mut String::new());

        while self.running {
            let current_time = self.start_time.elapsed().as_secs_f64();
            let delta_time = self.last_update.elapsed().as_secs_f64();
            self.last_update = Instant::now();

            // Generate new tasks
            let new_tasks = self.task_generator.generate_tasks(delta_time, current_time);
            
            // Distribute tasks to servers
            for task in new_tasks {
                let server_idx = select_least_loaded_server(&self.servers);
                self.servers[server_idx].add_task(task);
            }

            // Process server queues
            for server in &mut self.servers {
                if let Some(completed_task) = server.process_step(current_time) {
                    self.completed_tasks.push(completed_task);
                }
            }

            // Keep only recent completed tasks for statistics
            if self.completed_tasks.len() > 1000 {
                self.completed_tasks.drain(0..500);
            }

            // Calculate statistics
            let stats = calculate_statistics(&self.servers, &self.completed_tasks, delta_time);

            // Update display
            self.display.clear_screen();
            self.display.draw_header(
                self.servers.len(),
                self.task_generator.arrival_rate,
                self.task_generator.processing_time,
            );
            self.display.draw_server_queues(&self.servers);
            self.display.draw_statistics(&stats);
            self.display.draw_commands();

            // Handle user input
            self.handle_input();

            thread::sleep(Duration::from_millis(100)); // 10 FPS update rate
        }
    }

    fn handle_input(&mut self) {
        // Non-blocking input check
        if let Ok(ready) = crossterm::event::poll(Duration::from_millis(0)) {
            if ready {
                if let Ok(event) = crossterm::event::read() {
                    if let crossterm::event::Event::Key(key) = event {
                        match key.code {
                            crossterm::event::KeyCode::Char('q') => {
                                self.running = false;
                            }
                            crossterm::event::KeyCode::Char('s') => {
                                // Would need to implement number parsing
                                println!("Enter number of servers:");
                                let mut input = String::new();
                                if io::stdin().read_line(&mut input).is_ok() {
                                    if let Ok(num) = input.trim().parse::<usize>() {
                                        self.servers = (0..num).map(ServerState::new).collect();
                                        println!("Updated servers to {}", num);
                                    }
                                }
                            }
                            crossterm::event::KeyCode::Char('r') => {
                                println!("Enter arrival rate:");
                                let mut input = String::new();
                                if io::stdin().read_line(&mut input).is_ok() {
                                    if let Ok(rate) = input.trim().parse::<f64>() {
                                        self.task_generator.arrival_rate = rate.max(0.1);
                                        println!("Updated arrival rate to {}", rate);
                                    }
                                }
                            }
                            crossterm::event::KeyCode::Char('p') => {
                                println!("Enter processing time:");
                                let mut input = String::new();
                                if io::stdin().read_line(&mut input).is_ok() {
                                    if let Ok(time) = input.trim().parse::<f64>() {
                                        self.task_generator.processing_time = time.max(0.1);
                                        println!("Updated processing time to {}", time);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut servers = 3;
    let mut arrival_rate = 2.0;
    let mut processing_time = 1.0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--servers" => {
                if i + 1 < args.len() {
                    servers = args[i + 1].parse().unwrap_or(3);
                    i += 1;
                }
            }
            "--arrival-rate" => {
                if i + 1 < args.len() {
                    arrival_rate = args[i + 1].parse().unwrap_or(2.0);
                    i += 1;
                }
            }
            "--processing-time" => {
                if i + 1 < args.len() {
                    processing_time = args[i + 1].parse().unwrap_or(1.0);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let mut simulator = Simulator::new(servers, arrival_rate, processing_time);
    simulator.run();
}
