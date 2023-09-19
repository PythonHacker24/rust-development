extern crate sys_info;
extern crate chrono;

use sys_info::{mem_info, loadavg};
use chrono::Local;

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
        
  "#, time_info);

  println!("{}", logo_string);
}

fn main() {

    logo();
    let mem = mem_info().expect("Failed to retrieve memory info");

    println!("Total RAM: {} KB", mem.total);
    println!("Free RAM: {} KB", mem.free);
    println!("Used RAM: {} KB", mem.total - mem.free);

    let load = loadavg().expect("Failed to retrieve load average");

    println!("Load average (1 minute): {}", load.one);
    println!("Load average (5 minutes): {}", load.five);
    println!("Load average (15 minutes): {}", load.fifteen);

}

