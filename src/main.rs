use prex::procs::read_proc_fs;

fn main() {
    let proc_list = read_proc_fs().unwrap();

    println!("PID\t\tPPID\t\tNAME");

    for p in proc_list.iter() {
        println!("{}\t\t{}\t\t{}", p.pid(), p.ppid(), p.name());
    }
}
