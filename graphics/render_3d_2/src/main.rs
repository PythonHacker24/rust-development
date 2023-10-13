use std::process::Command;
use std::{thread, time::Duration};

fn rotate_x(x: i32, y: i32, theta: f64) -> f64 {
    let theta_radians = theta.to_radians();
    let sine_value = f64::sin(theta_radians);
    let cosine_value = f64::cos(theta_radians);
    return x as f64 * cosine_value - y as f64 * sine_value;
} 

fn rotate_y(x: i32, y: i32, theta: f64) -> f64{
    let theta_radians = theta.to_radians();
    let sine_value = f64::sin(theta_radians);
    let cosine_value = f64::cos(theta_radians);
    return x as f64 * sine_value + y as f64 * cosine_value;
}

fn space_frame(horizontal_data: Vec<i32>, vertical_data: Vec<i32>) -> Vec<Vec<char>> {

    // horizontal_data: [ 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0 ]
    // vertical_data: [ 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]

    let void_element: char = '*';
    let filled_element: char = '|';

    let mut frame_vector: Vec<Vec<char>> = Vec::new();
    for vertical_element in vertical_data {
        let mut temp_vertical_vector: Vec<char> = Vec::new();
        if vertical_element == 0 {
            temp_vertical_vector.push(void_element);
        } else if vertical_element == 1 {
            for horizontal_element in &horizontal_data {
                if *horizontal_element == 0 {
                    temp_vertical_vector.push(void_element);
                } else if *horizontal_element == 1 {
                    temp_vertical_vector.push(filled_element);
                }
            }
        }
        frame_vector.push(temp_vertical_vector.clone());
    }

    return frame_vector;
}

fn horizontal_data_function(x2: i32, x1: i32, width: i32, counter: i32) -> Vec<i32> {
    // width includes the bounds of the space 
    let mut data_vector: Vec<i32> = Vec::new();
    for value in 1..=width {
        if value > x1 + counter && value < x2 - counter { // x2 > counter
            data_vector.push(1);
        } else {
            data_vector.push(0);
        }
    }
    return data_vector;
}

fn vertical_data_function(y2: i32, y1: i32, height: i32, counter: i32) -> Vec<i32> {
    // height includes the bounds of the space
    let mut data_vector: Vec<i32> = Vec::new();
    for value in 1..=height {
        if value > y1 && value < y2 {
            data_vector.push(1);
        } else {
            data_vector.push(0);
        }
    }
    return data_vector;
} 

fn main() {
    // Defining space parameters 
    let width = 50;
    let height = 20;

    // Defining square parameters
    let x2 = 30;
    let x1 = 10;
    let y2 = 15;
    let y1 = 5;
    
    loop {
        let mut print_sequence = Vec::new();
        for i in 1..=10 {
            print_sequence.push(i);
        }

        for i in (1..=10).rev() {
            print_sequence.push(i);
        }
    
        for counter_instance in print_sequence {
            let vertical_data = vertical_data_function(y2, y1, height, counter_instance);
            let horizontal_data = horizontal_data_function(x2, x1, width, counter_instance);
            let mut space: Vec<Vec<char>> = Vec::new();
            space = space_frame(horizontal_data, vertical_data);
    
            for horizontal_vector in &space {
                for pixel in horizontal_vector {
                    print!("{}", pixel); 
                }
                print!("\n");
            }
            thread::sleep(Duration::from_millis(10));
            Command::new("clear").status().expect("Scrren clear failed!");
        }
    }
}
