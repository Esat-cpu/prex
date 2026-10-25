use std::fs;
use std::io;

pub struct Process {
    pub name: String,

    pub pid: i32,
    pub ppid: i32,
}

impl Process {
    pub fn new(pid: i32) -> io::Result<Process> {
        let status = fs::read_to_string(format!("/proc/{pid}/status"))?;

        let mut name = "?".to_owned();
        let mut ppid = -1;

        for line in status.lines() {
            if let Some(s) = line.strip_prefix("Name:") {
                name = s.trim().to_owned();
            } else if let Some(s) = line.strip_prefix("PPid:") {
                ppid = s.trim().parse().unwrap_or(-1);
            }
        }

        Ok(Process { name, pid, ppid })
    }
}

pub fn read_proc_fs() -> io::Result<Vec<Process>> {
    let mut proc_list: Vec<Process> = Vec::new();

    for entry in fs::read_dir("/proc")? {
        let entry = entry?.file_name();
        let entry = entry.to_str();

        match entry {
            Some(e) if let Ok(pid) = e.parse::<i32>() => {
                if let Ok(p) = Process::new(pid) {
                    proc_list.push(p);
                }
            }
            _ => (),
        }
    }

    Ok(proc_list)
}
