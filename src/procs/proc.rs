//! Reads process information from `/proc`.
//!
//! Currently only `/proc/<pid>/stat` is parsed.

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
    /// Create a process instance with given PID according to `/proc/<pid>/stat`
    pub fn new(pid: i32) -> io::Result<Process> {
        let stat = fs::read_to_string(format!("/proc/{pid}/stat"))?;
        parse_stat(pid, stat.as_str())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> &str {
        &self.state
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

/// Parse given stat content
fn parse_stat(pid: i32, stat: &str) -> io::Result<Process> {
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

/// Reads every process currently listed in `/proc`.
///
/// Processes that exit mid-read or can't be parsed are skipped.
pub fn read_proc_fs() -> io::Result<Vec<Process>> {
    let mut proc_list: Vec<Process> = Vec::new();

    for entry in fs::read_dir("/proc")? {
        let entry = entry?.file_name();
        let entry = entry.to_str();

        if let Some(e) = entry
            && let Ok(pid) = e.parse::<i32>()
            && let Ok(p) = Process::new(pid)
        {
            proc_list.push(p);
        }
    }

    Ok(proc_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_with_parens_and_spaces() {
        let p = parse_stat(42, "42 (a (b) c) S 7 9 9").unwrap();
        assert_eq!(p.name(), "a (b) c");
        assert_eq!(p.state(), "S");
        assert_eq!(p.ppid(), 7);
        assert_eq!(p.pgrp(), 9);
    }

    #[test]
    fn truncated_line_is_an_error() {
        assert!(parse_stat(42, "42 (a) S").is_err());
    }
}
