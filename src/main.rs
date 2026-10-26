use clap::Parser;
use prex::{Column, procs};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Instances that will show up
    #[arg(long, value_enum, value_delimiter= ',',
            default_values_t = [Column::Pid, Column::Ppid, Column::Name])]
    show: Vec<Column>,

    #[arg(short, long, default_value_t = 12)]
    column_width: usize,
}

fn main() {
    let args = Args::parse();
    let proc_list = procs::read_proc_fs().unwrap();

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
