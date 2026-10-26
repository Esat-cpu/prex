pub mod procs;

use clap::ValueEnum;
use procs::Process;

#[derive(Clone, Copy, ValueEnum)]
pub enum Column {
    Pid,
    Ppid,
    Pgrp,
    Name,
    State,
}

impl Column {
    pub fn header(self) -> &'static str {
        match self {
            Column::Pid => "PID",
            Column::Ppid => "PPID",
            Column::Pgrp => "PGRP",
            Column::Name => "NAME",
            Column::State => "STATE",
        }
    }

    pub fn value(self, p: &Process) -> String {
        match self {
            Column::Pid => p.pid().to_string(),
            Column::Ppid => p.ppid().to_string(),
            Column::Pgrp => p.pgrp().to_string(),
            Column::Name => p.name().to_owned(),
            Column::State => p.state().to_owned(),
        }
    }
}
