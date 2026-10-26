use std::fs;
use std::io;

pub struct Process {
    name: String,

    state: String,

    pid: i32,
    ppid: i32,
    pgrp: i32,
}

impl Process {
    pub fn new(pid: i32) -> io::Result<Process> {
        let stat = fs::read_to_string(format!("/proc/{pid}/stat"))?;

        let (Some(open), Some(close)) = (stat.find('('), stat.rfind(')')) else {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        };

        let name = stat[(open + 1)..close].to_owned();

        let fields: Vec<&str> = stat[(close + 1)..].split_whitespace().collect();
        // Field number according to proc_pid_stat(5)
        let field = |n: usize| fields.get(n - 3).copied();

        let (Some(state), Some(ppid), Some(pgrp)) = (
            field(3).map(String::from),
            field(4).and_then(|s| s.parse().ok()),
            field(5).and_then(|s| s.parse().ok()),
        ) else {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        };

        Ok(Process {
            name,
            state,
            pid,
            ppid,
            pgrp,
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn state(&self) -> String {
        self.state.clone()
    }

    pub fn pid(&self) -> i32 {
        self.pid
    }

    pub fn ppid(&self) -> i32 {
        self.ppid
    }

    pub fn pgrp(&self) -> i32 {
        self.pgrp
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
