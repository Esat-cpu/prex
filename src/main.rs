use clap::Parser;
use prex::{Column, procs};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Instances that will show up
    #[arg(long, value_enum, value_delimiter= ',',
            default_values_t = [Column::Pid, Column::Ppid, Column::Name])]
    show: Vec<Column>,
}

fn main() {
    let args = Args::parse();
    let proc_list = procs::read_proc_fs().unwrap();

    for field in &args.show {
        print!("{:<12}", field.header());
    }
    println!();

    for p in &proc_list {
        for field in &args.show {
            print!("{:<12}", field.value(p));
        }
        println!();
    }
}
