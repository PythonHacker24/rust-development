use std::process::Command;
use std::{thread, time::Duration}; 

#[derive(Debug)]
struct GeometricBounds {
    x2: i32,
    x1: i32,
    y2: i32,
    y1: i32,
}

fn display(space_vec: Vec<Vec<char>>) {
    for vector in space_vec {
        for pixel in vector {
            print!("{}", pixel);
        }
        print!("\n");
    }
}

fn frame(width: i32, height: i32, frame_count: i32) -> Vec<Vec<char>> {
    let mut space_vector: Vec<Vec<char>> = Vec::new();
    let mut width_vector: Vec<char> = Vec::new();
    
    let x2 = 35;
    let x1 = 15;
    let y2 = 15;
    let y1 = 5;

    for pixel in 1..=width { 
        if pixel < x2 && pixel > x1 {
            width_vector.push('+');
        } else {
            width_vector.push(' ');
       }
    }

    for _ in 1..=height {
        space_vector.push(width_vector.clone())
    }

    return space_vector;
}

fn main() {
    
    // Setting up display configurations 
    let width: i32 = 50;
    let height: i32 = 20;
    
    for count in 1..50 {

        let space_vector = frame(width, height, count);
        for horizontal_vector in space_vector {
            for pixel in horizontal_vector {
                print!("{}", pixel);
            }
            print!("\n");
        }
        thread::sleep(Duration::from_millis(100));
        Command::new("clear").status().expect("Screen clear failed!");
    }
}
