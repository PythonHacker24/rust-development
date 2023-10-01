extern crate sys_info;
extern crate chrono;
extern crate colored;

use sys_info::{mem_info, loadavg};
use chrono::Local;
use colored::*;
use std::process::Command;

fn date_and_time() -> String {
    
    let current_datetime = Local::now();
    let formatted_datetime = current_datetime.format("%d-%m-%Y %H:%M:%S");

    return formatted_datetime.to_string();
}

fn logo() {
    
    let time_info = date_and_time(); 
    let logo_string = format!(r#"
                 ⣀⣀⠀⠀⠀⠀⠀⠀
  ⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⢀⣴⣿⣿⡿⠀⠀⠀⠀⠀⠀
  ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀      Date: {}
  ⠀⠀⠀⢀⣠⣤⣤⣤⣀⣀⠈⠋⠉⣁⣠⣤⣤⣤⣀⡀⠀⠀
  ⠀⢠⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀      Aditya Patil
  ⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠋⠀
  ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠀⠀⠀      Device: Apple Macbook Air M2 8GB RAM
  ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀
  ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀      Designed by Apple in California
  ⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⣀
  ⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁      Developed by: Steve Jobs
  ⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀                    Steve Wozniak
  ⠀⠀⠀⠈⠙⢿⣿⣿⣿⠿⠟⠛⠻⠿⣿⣿⣿⡿⠋⠀⠀⠀                    Ronald Wayne
        
  "#, time_info).white().bold();

  println!("{}", logo_string);
}

fn plot(value: &mut u64, total_value: &mut u64) {

    let percentage = (*value / *total_value) * 100;
    print!("[ ");
    let bar_char = "█".cyan();
    let bar = &bar_char;

    for _ in 0..=(percentage / 2) {
        print!("{}", *bar);
    }

    for _ in 0..=(50 - (percentage / 2)) {
        print!("-");
    }

    println!(" ] {}%", percentage);
}

fn system_info() {

    let mut hostname_cmd = Command::new("uname");
    let mut hardware_cmd = Command::new("uname");
    let mut version_cmd = Command::new("uname");
    
    hostname_cmd.arg("-n");
    hardware_cmd.arg("-m");
    version_cmd.arg("-v");

    let hostname_output = hostname_cmd.output().expect("Failed to get the hostname information");
    let hardware_output = hardware_cmd.output().expect("Failed to get the hardware information");
    let version_output = version_cmd.output().expect("Failed to get the version of the operating system");

    if hostname_output.status.success() {

        let stdout = String::from_utf8_lossy(&hostname_output.stdout);
        println!("Hostname: {}", stdout);
    } else {

        let stderr = String::from_utf8_lossy(&hostname_output.stderr);
        println!("Hostname Fetch Error: {}", stderr);
    }

    if hardware_output.status.success() {

        let stdout = String::from_utf8_lossy(&hardware_output.stdout);
        println!("Processor: {}", stdout);
    } else {

        let stderr = String::from_utf8_lossy(&hardware_output.stderr);
        println!("Processor Fetch Error: {}", stderr);
    }

    if version_output.status.success() {

        let stdout = String::from_utf8_lossy(&version_output.stdout);
        println!("Version Information: {}", stdout);
    } else {

        let stderr = String::from_utf8_lossy(&version_output.stderr);
        println!("Version Information Fetch Error: {}", stderr);
    }
}

fn main() {

    logo();
    let mem = mem_info().expect("Failed to retrieve memory info");
    
    let sample_text = "[+] Apple Banner by Aditya. Developed in RUST\n".bold().cyan();
   
    system_info();
    println!("{}", sample_text);
    
    let mut used_mem = (mem.total - mem.free) / 1024;
    let mut total_mem = mem.total / 1024;
    print!("Ram Usage: ");
    plot(&mut used_mem, &mut total_mem);

    println!("\nTotal RAM: {} MB", mem.total / 1024);
    println!("Free RAM: {} MB", mem.free / 1024);
    println!("Used RAM: {} MB", (mem.total - mem.free) / 1024);

    let load = loadavg().expect("Failed to retrieve load average");

    println!("Load average (1 minute): {}", load.one);
    println!("Load average (5 minutes): {}", load.five);
    println!("Load average (15 minutes): {} \n", load.fifteen);

}

