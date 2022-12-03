use std::collections::HashMap;
use sysinfo::{self, System, SystemExt};

use crate::process::{self, ProcessData};

pub struct Sysinfo {
    prev_idle: f64,
    prev_non_idle: f64,
    cpu_times: HashMap<i32, u64>,
    pub total_memory_bytes: u64,
}

impl Sysinfo {
    pub fn new() -> Self {
        let mut system = System::new_with_specifics(sysinfo::RefreshKind::new());
        system.refresh_memory();

        Sysinfo {
            prev_idle: 0.0,
            prev_non_idle: 0.0,
            cpu_times: HashMap::new(),
            total_memory_bytes: system.total_memory(),
        }
    }

    pub fn read_process_data(&mut self, data: &mut HashMap<i32, ProcessData>) -> () {
        let processes = process::read_process_data(
            data,
            &mut self.prev_idle,
            &mut self.prev_non_idle,
            &mut self.cpu_times,
            self.total_memory_bytes,
        );

        processes
    }
}
