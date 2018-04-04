extern crate colored;
extern crate system_information;
#[macro_use]
extern crate clap;

use colored::*;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut human_readable: bool = true;

    if matches.occurrences_of("raw") != 0 {
        human_readable = false;
    }

    let mut default_fs = "";
    if cfg!(unix) {
        default_fs = "/";
    }
    if cfg!(windows) {
        default_fs = "C";
    }
    let fs = matches.value_of("filesystem").unwrap_or(default_fs);

    print_header();
    print_os();
    print_disk_info(human_readable, fs);
    print_mem_info(human_readable);
    print_cpu_info();
}

fn print_header() {
    let user = system_information::get_username().unwrap();
    let hostname = system_information::get_hostname().unwrap();

    let comp_info = format!("{}@{}", user.blue().bold(), hostname.blue().bold());

    println!("{}", comp_info);
}

fn print_os() {
    let os = system_information::get_os();

    println!("{}: {:?}", "OS".blue().bold(), os.os_type);
    println!("{}: {:?}", "Version".blue().bold(), os.version);
}

fn print_disk_info(human_readable: bool, fs: &str) {
    if human_readable {
        let disk = system_information::get_readable_disk_info(fs);

        println!("{}: {}", "Total Disk Space".blue().bold(), disk[0]);
        println!("{}: {}", "Available Disk Space".blue().bold(), disk[1]);
        println!("{}: {}", "In Use Disk Space".blue().bold(), disk[2]);
    }
    else {
        let disk = system_information::get_disk_info(fs);

        println!("{}: {} bytes", "Total Disk Space".blue().bold(), disk.total.unwrap());
        println!("{}: {} bytes", "Available Disk Space".blue().bold(), disk.free.unwrap());
        println!("{}: {} bytes", "In Use Disk Space".blue().bold(), disk.in_use.unwrap());
    }
}

fn print_mem_info(human_readable: bool) {
    if human_readable {
        let mem = system_information::get_readable_mem_info();

        println!("{}: {}", "Total Memory".blue().bold(), mem[0]);
        println!("{}: {}", "Available Memory".blue().bold(), mem[1]);
        println!("{}: {}", "In Use Memory".blue().bold(), mem[2]);
    }
    else {
        let mem = system_information::get_mem_info();

        println!("{}: {} bytes", "Total Memory".blue().bold(), mem.total.unwrap());
        println!("{}: {} bytes", "Available Memory".blue().bold(), mem.free.unwrap());
        println!("{}: {} bytes", "In Use Memory".blue().bold(), mem.in_use.unwrap());
    }
}

fn print_cpu_info() {
    let cpu = system_information::get_cpu_info();

    println!("{}: {}", "CPU Model".blue().bold(), cpu.model.unwrap());
    println!("{}: {}", "CPU Cores".blue().bold(), cpu.num.unwrap());
    println!("{}: {} mhz", "CPU Speed".blue().bold(), cpu.mhz.unwrap());
}
