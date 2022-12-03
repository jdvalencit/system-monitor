use procfs::{
    process::{self, Process, Stat},
    CpuTime, KernelStats,
};
use std::collections::{HashMap, HashSet};
use sysinfo::ProcessStatus;

const MAX_STAT_NAME_LEN: usize = 12;

#[derive(Debug, Clone, Default)]
pub struct ProcessData {
    pub pid: i32,
    pub status: String,
    pub parent_pid: i32,
    pub name: String,
    pub command: String,
    pub cpu_usage_percent: f64,
    pub mem_usage_percent: f64,
    pub disk_read_bytes: Option<u64>,
    pub disk_write_bytes: Option<u64>,
    pub uid: Option<u32>,
    pub priority: i64,
}

impl ProcessData {
    fn new(
        process: Process,
        stat: Stat,
        cpu_usage: f64,
        cpu_fraction: f64,
        prev_cpu_time: u64,
        total_memory_bytes: u64,
    ) -> (Self, u64) {
        let (command, name) = get_process_strings(&process, &stat);
        let (cpu_usage_percent, new_process_time) =
            get_final_usage(&stat, cpu_usage, cpu_fraction, prev_cpu_time);

        let mut total_disk_read_bytes = None;
        let mut total_disk_write_bytes = None;

        if let Ok(_) = process.io() {
            total_disk_read_bytes = Some(disk_usage(&process).0);
            total_disk_write_bytes = Some(disk_usage(&process).1);
        }

        let mem_usage_percent = memory_usage(&stat, total_memory_bytes);

        let data = ProcessData {
            pid: process.pid,
            parent_pid: stat.ppid,
            cpu_usage_percent,
            mem_usage_percent,
            priority: stat.priority,
            disk_read_bytes: total_disk_read_bytes,
            disk_write_bytes: total_disk_write_bytes,
            name: name,
            command: command,
            uid: process.uid().ok(),
            status: ProcessStatus::from(stat.state).to_string(),
        };

        (data, new_process_time)
    }
}

pub fn read_process_data(
    data: &mut HashMap<i32, ProcessData>,
    prev_idle: &mut f64,
    prev_non_idle: &mut f64,
    cpu_times: &mut HashMap<i32, u64>,
    total_memory_bytes: u64,
) -> () {
    let mut current_pids = HashSet::new();

    let (cpu_usage, cpu_percentage) = calculate_usage(prev_idle, prev_non_idle);

    let proc_list = process::all_processes().unwrap();

    for element in proc_list {
        let proc = element.unwrap();
        let stat = proc.stat().unwrap();
        let pid = proc.pid;
        let prev_proc_cpu_time = *cpu_times.get(&pid).unwrap_or(&0);

        let (process_data, new_process_cpu_times) = ProcessData::new(
            proc,
            stat,
            cpu_usage,
            cpu_percentage,
            prev_proc_cpu_time,
            total_memory_bytes,
        );

        cpu_times.insert(pid, new_process_cpu_times);
        current_pids.insert(pid);
        data.insert(pid, process_data);
    }

    let all_pids: HashSet<i32> = cpu_times.keys().map(|k| *k).collect();
    all_pids.difference(&current_pids).for_each(|k| {
        cpu_times.remove(&k);
    });
}

fn get_process_strings(proc: &Process, stat: &Stat) -> (String, String) {
    let (command, name) = {
        let truncated_name = stat.comm.as_str();
        if let Ok(cmdline) = proc.cmdline() {
            if cmdline.is_empty() {
                return (format!("[{}]", truncated_name), truncated_name.to_string());
            } else {
                let name = if truncated_name.len() >= MAX_STAT_NAME_LEN {
                    if let Some(first_part) = cmdline.first() {
                        first_part
                            .rsplit_once('/')
                            .map(|(_prefix, suffix)| suffix)
                            .unwrap_or(truncated_name)
                            .to_string()
                    } else {
                        truncated_name.to_string()
                    }
                } else {
                    truncated_name.to_string()
                };

                return (cmdline.join(" "), name);
            }
        } else {
            (truncated_name.to_string(), truncated_name.to_string())
        }
    };

    return (command, name);
}

//
// CPU UTILITIES
//

fn get_final_usage(
    stat: &Stat,
    cpu_usage: f64,
    cpu_fraction: f64,
    prev_proc_times: u64,
) -> (f64, u64) {
    let new_proc_times = stat.utime + stat.stime;
    let diff = (new_proc_times - prev_proc_times) as f64;

    if cpu_usage == 0.0 {
        (0.0, new_proc_times)
    } else {
        (diff / cpu_usage * 100_f64 * cpu_fraction, new_proc_times)
    }
}

fn calculate_usage(prev_idle: &mut f64, prev_non_idle: &mut f64) -> (f64, f64) {
    let kernel_stat = KernelStats::new().unwrap();
    let total_time = kernel_stat.total;
    let (idle, non_idle) = get_idle_times(total_time);

    let total = idle + non_idle;
    let prev_total = *prev_idle + *prev_non_idle;

    let total_delta: f64 = total - prev_total;
    let idle_delta: f64 = idle - *prev_idle;

    *prev_idle = idle;
    *prev_non_idle = non_idle;

    let result = if total_delta - idle_delta != 0_f64 {
        total_delta - idle_delta
    } else {
        1_f64
    };

    let cpu_porcentage = if total_delta != 0_f64 {
        result / total_delta
    } else {
        0_f64
    };

    (result, cpu_porcentage)
}

fn get_idle_times(times: CpuTime) -> (f64, f64) {
    let user = times.user;
    let nice = times.nice;
    let system = times.system;
    let idle = times.user;
    let iowait = times.iowait.unwrap();
    let irq = times.irq.unwrap();
    let softirq = times.softirq.unwrap();
    let steal = times.steal.unwrap();

    let idle = idle + iowait;
    let non_idle = user + nice + system + irq + softirq + steal;

    (idle as f64, non_idle as f64)
}

//
// DISKS UTILITIES
//

pub fn disk_usage(process: &Process) -> (u64, u64) {
    let process = process.io().unwrap();

    let read_bytes = process.read_bytes;
    let written_bytes = process.write_bytes;

    (read_bytes, written_bytes)
}

//
// MEMORY UTILITIES
//

pub fn memory_usage(stat: &Stat, total_memory_bytes: u64) -> f64 {
    let mem_usage_bytes = u64::try_from(stat.rss_bytes().unwrap_or(0)).unwrap_or(0);
    let mem_usage_percent = mem_usage_bytes as f64 / total_memory_bytes as f64 * 100.0;

    mem_usage_percent
}
