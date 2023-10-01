extern crate sysinfo;

use std::process::Command;
use sysinfo::{CpuExt, System, SystemExt};
use colored::*;

fn return_bar(percent: &mut f32) -> ColoredString {
   if *percent < 25.0 {
        let bar_char = "■".cyan();
        return bar_char;
   } else if *percent > 25.0 && *percent < 80.0 {
       let bar_char = "■".yellow();
       return bar_char;
   } else if *percent > 80.0 {
       let bar_char = "■".red();
       return bar_char;
   }
   return "■".white();
}

fn plot(percent: f32) {

    // let bar_char = "█";
    let mut percent_mut = percent;
    let bar_char = return_bar(&mut percent_mut); 

    for _ in 0..=((percent/2.0) as i32) {
        print!("{}", bar_char);
    }
    
    for _ in 0..=((50.0 - (percent/2.0)) as i32) {
        print!(" ");
    }
    
    let colored_percent = format!(r#"{}"#, percent).white().bold();
    print!("{}", colored_percent);
    // print!(" ]");
}

fn post_inc(counter: &mut i64) {
    *counter += 1;
}

fn main () {

    let mut sys = System::new();

    loop {

        let mut counter = 1;
        sys.refresh_cpu(); // Refreshing CPU information.
        for cpu in sys.cpus() {
            // print!("{}% ", cpu.cpu_usage());
            let count = format!(r#"CPU: {} "#, counter).green().bold();
            print!("{}", count);
            plot(cpu.cpu_usage());
            print!("\n");
            post_inc(&mut counter);
        }

        print!("\n\n");
        Command::new("clear").status().expect("Screen Clear Failed!");
        // Sleeping to let time for the system to run for long
        // enough to have useful information.
        std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
