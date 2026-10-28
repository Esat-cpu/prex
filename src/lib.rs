pub mod procs;

use std::collections::HashMap;

use clap::{Parser, ValueEnum};
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

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Instances that will show up
    #[arg(long, value_enum, value_delimiter= ',',
            default_values_t = [Column::Pid, Column::Ppid, Column::Name])]
    pub show: Vec<Column>,

    #[arg(short, long, default_value_t = 12)]
    pub column_width: usize,

    /// Print process tree
    #[arg(short, long)]
    pub tree: bool,
}

pub fn print_columns(args: Args, proc_list: Vec<Process>) {
    for field in &args.show {
        print!("{:<width$}", field.header(), width = args.column_width);
    }
    println!();

    for p in &proc_list {
        for field in &args.show {
            print!("{:<width$}", field.value(p), width = args.column_width);
        }
        println!();
    }
}

fn print_tree_impl(
    prefix: &str,
    start_pid: i32,
    hm: &HashMap<i32, Vec<usize>>,
    proc_list: &[Process],
) {
    let Some(children) = hm.get(&start_pid) else {
        return;
    };

    for (i, &child) in children.iter().enumerate() {
        let p = &proc_list[child];
        let last_child = (i + 1) == children.len();
        let prefix2 = if start_pid != 0 {
            if last_child { "└─" } else { "├─" }
        } else {
            ""
        };

        println!("{prefix}{prefix2}{}({})", p.name(), p.pid());

        let prefix = if start_pid != 0 {
            format!("{prefix}{}", if last_child { "      " } else { "│     " })
        } else {
            " ".repeat(6)
        };
        print_tree_impl(&prefix, p.pid(), hm, proc_list);
    }
}

pub fn print_tree(proc_list: &[Process]) {
    // Maps a parent PID to the indices (in `proc_list`) of its children
    let mut hm: HashMap<i32, Vec<usize>> = HashMap::new();

    for (i, p) in proc_list.iter().enumerate() {
        hm.entry(p.ppid()).or_default().push(i);
    }

    print_tree_impl("", 0, &hm, proc_list);
}
