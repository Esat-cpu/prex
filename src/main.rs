use clap::Parser;
use prex::procs;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Instances that will show up
    #[arg(long, default_value = "pid,ppid,name")]
    show: String,
}

fn main() {
    let args = Args::parse();

    let proc_list = procs::read_proc_fs().unwrap();

    let mut label = String::new();

    type Getter = Box<dyn Fn(&procs::Process) -> String + 'static>;
    let mut getters: Vec<Getter> = Vec::new();

    let fields = args.show.split(',');

    for field in fields {
        match field {
            "pid" => {
                label.push_str("PID\t\t");
                getters.push(Box::new(|p| format!("{}", p.pid())));
            }
            "ppid" => {
                label.push_str("PPID\t\t");
                getters.push(Box::new(|p| format!("{}", p.ppid())));
            }
            "name" => {
                label.push_str("NAME\t\t");
                getters.push(Box::new(|p| p.name().to_owned()));
            }
            "state" => {
                label.push_str("STATE\t\t");
                getters.push(Box::new(|p| p.state().to_owned()));
            }
            "pgrp" => {
                label.push_str("PGRP\t\t");
                getters.push(Box::new(|p| format!("{}", p.pgrp())));
            }
            _ => {
                eprintln!("Wrong field name");
                std::process::exit(1);
            }
        }
    }

    println!("{}", label);

    for p in proc_list.iter() {
        for get in &getters {
            print!("{}\t\t", get(p));
        }
        println!();
    }
}
