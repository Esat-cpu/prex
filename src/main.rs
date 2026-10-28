use clap::Parser;
use prex::{Args, print_columns, print_tree, procs::read_proc_fs};

fn main() {
    let args = Args::parse();
    let proc_list = read_proc_fs().unwrap();

    if args.tree {
        print_tree(&proc_list);
    } else {
        print_columns(args, proc_list);
    }
}
