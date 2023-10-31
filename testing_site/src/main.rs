extern crate crossterm;

use std::thread;
use std::time::Duration;
use crossterm::{
    execute,
    cursor,
    terminal::{ClearType, Clear, size},
    ExecutableCommand,
};

fn main() {
    let mut lines: Vec<String> = Vec::new();
    lines.push("Line 1".to_string());
    lines.push("Line 2".to_string());
    lines.push("Line 3".to_string());
    lines.push("Line 4".to_string());
    lines.push("Line 5".to_string());
    lines.push("Line 6".to_string());
    lines.push("Line 7".to_string());
    lines.push("Line 8".to_string());
    lines.push("Line 9".to_string());
    lines.push("Line 10".to_string());
    lines.push("Line 11".to_string());
    lines.push("Line 12".to_string());
    
    loop {
        print_at_top(&lines);

        // Sleep for a while to control the scrolling speed
        thread::sleep(Duration::from_millis(500));

        // Shift the lines to create a scrolling effect
        let last_line = lines.pop().unwrap();
        lines.insert(0, last_line);
    }
}

fn print_at_top(lines: &Vec<String>) {
    // Clear the terminal
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();

    // Get the terminal size
    let (width, _height) = size().unwrap();

    // Calculate the number of lines to print based on terminal height
    let num_lines_to_print = lines.len().min(_height as usize);

    // Move the cursor to the top of the terminal
    execute!(std::io::stdout(), cursor::MoveTo(0, 0)).unwrap();

    // Print the lines at the top of the terminal
    for line in &lines[0..num_lines_to_print] {
        println!("{}", line);
    }
}

