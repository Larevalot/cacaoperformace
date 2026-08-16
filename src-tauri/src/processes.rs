use serde::{Deserialize, Serialize};
use sysinfo::{Pid, System};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessItem {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_mb: f32,
    pub status: String,
    pub exe_path: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessDetails {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub memory_mb: f32,
    pub status: String,
    pub exe_path: String,
    pub command: Vec<String>,
    pub parent_pid: Option<u32>,
    pub start_time_secs: u64,
    pub user_id: String,
}

pub fn get_process_list(sys: &mut System) -> Vec<ProcessItem> {
    sys.refresh_processes();

    let mut list: Vec<ProcessItem> = sys
        .processes()
        .iter()
        .map(|(pid, proc)| {
            let pid_u32 = pid.as_u32();
            let name = proc.name().to_string();
            let cpu_usage = (proc.cpu_usage() * 10.0).round() / 10.0;
            let mem_mb = ((proc.memory() as f64) / (1024.0 * 1024.0) * 10.0).round() / 10.0;
            let status = format!("{:?}", proc.status());
            let exe_path = proc
                .exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            let user_id = proc
                .user_id()
                .map(|u| u.to_string())
                .unwrap_or_else(|| "system".to_string());

            ProcessItem {
                pid: pid_u32,
                name,
                cpu_usage,
                memory_mb: mem_mb as f32,
                status,
                exe_path,
                user_id,
            }
        })
        .collect();

    // Sort by CPU usage descending by default
    list.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
    list
}

pub fn fetch_process_details(sys: &mut System, target_pid: u32) -> Option<ProcessDetails> {
    sys.refresh_processes();
    let sys_pid = Pid::from_u32(target_pid);

    if let Some(proc) = sys.processes().get(&sys_pid) {
        let command: Vec<String> = proc.cmd().iter().map(|s| s.to_string()).collect();
        let parent_pid = proc.parent().map(|p| p.as_u32());

        Some(ProcessDetails {
            pid: target_pid,
            name: proc.name().to_string(),
            cpu_usage: (proc.cpu_usage() * 10.0).round() / 10.0,
            memory_bytes: proc.memory(),
            memory_mb: ((((proc.memory() as f64) / (1024.0 * 1024.0)) * 10.0).round() / 10.0) as f32,
            status: format!("{:?}", proc.status()),
            exe_path: proc
                .exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            command,
            parent_pid,
            start_time_secs: proc.start_time(),
            user_id: proc
                .user_id()
                .map(|u| u.to_string())
                .unwrap_or_else(|| "system".to_string()),
        })
    } else {
        None
    }
}

pub fn kill_target_process(sys: &mut System, target_pid: u32) -> bool {
    sys.refresh_processes();
    let sys_pid = Pid::from_u32(target_pid);

    if let Some(proc) = sys.processes().get(&sys_pid) {
        proc.kill()
    } else {
        false
    }
}
