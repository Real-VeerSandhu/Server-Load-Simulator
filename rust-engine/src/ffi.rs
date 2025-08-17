use crate::queue::{ServerState, Task, TaskGenerator, select_least_loaded_server};
use crate::statistics::{StatisticsSnapshot, calculate_statistics};
use std::slice;
use std::ffi::c_void;

#[repr(C)]
pub struct ProcessingResult {
    pub completed_tasks_count: usize,
    pub new_tasks_count: usize,
}

static mut TASK_GENERATOR: Option<TaskGenerator> = None;
static mut COMPLETED_TASKS: Vec<Task> = Vec::new();

#[no_mangle]
pub extern "C" fn init_engine(
    arrival_rate: f64,
    processing_time: f64,
    processing_variance: f64,
) {
    unsafe {
        TASK_GENERATOR = Some(TaskGenerator::new(arrival_rate, processing_time, processing_variance));
        COMPLETED_TASKS.clear();
    }
}

#[no_mangle]
pub extern "C" fn process_simulation_step(
    servers_ptr: *mut ServerState,
    server_count: usize,
    delta_time: f64,
    current_time: f64,
) -> ProcessingResult {
    if servers_ptr.is_null() {
        return ProcessingResult { completed_tasks_count: 0, new_tasks_count: 0 };
    }

    let servers = unsafe { slice::from_raw_parts_mut(servers_ptr, server_count) };
    
    // Generate new tasks
    let new_tasks = unsafe {
        if let Some(ref mut generator) = TASK_GENERATOR {
            generator.generate_tasks(delta_time, current_time)
        } else {
            Vec::new()
        }
    };

    // Distribute new tasks to servers
    for task in &new_tasks {
        let server_idx = select_least_loaded_server(servers);
        servers[server_idx].add_task(task.clone());
    }

    // Process existing tasks
    let mut completed_count = 0;
    for server in servers.iter_mut() {
        if let Some(completed_task) = server.process_step(current_time) {
            unsafe {
                COMPLETED_TASKS.push(completed_task);
            }
            completed_count += 1;
        }
    }

    ProcessingResult {
        completed_tasks_count: completed_count,
        new_tasks_count: new_tasks.len(),
    }
}

#[no_mangle]
pub extern "C" fn get_statistics(
    servers_ptr: *const ServerState,
    server_count: usize,
    time_window: f64,
) -> StatisticsSnapshot {
    if servers_ptr.is_null() {
        return StatisticsSnapshot::default();
    }

    let servers = unsafe { slice::from_raw_parts(servers_ptr, server_count) };
    let completed_tasks = unsafe { &COMPLETED_TASKS };
    
    calculate_statistics(servers, completed_tasks, time_window)
}

#[no_mangle]
pub extern "C" fn create_server(id: usize) -> *mut ServerState {
    let server = Box::new(ServerState::new(id));
    Box::into_raw(server)
}

#[no_mangle]
pub extern "C" fn free_server(server_ptr: *mut ServerState) {
    if !server_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(server_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn update_arrival_rate(new_rate: f64) {
    unsafe {
        if let Some(ref mut generator) = TASK_GENERATOR {
            generator.arrival_rate = new_rate;
        }
    }
}
