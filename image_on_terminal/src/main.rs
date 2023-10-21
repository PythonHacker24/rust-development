extern crate image; 

use std::env;
use image::GenericImageView;

fn image_to_vector(location: String) -> Vec<Vec<Vec<u8>>>{
    let img = image::open(location).unwrap();
    let (width, height) = img.dimensions();
    
    let mut output: Vec<Vec<Vec<u8>>> = Vec::new();
    for height_element in 0..height {
        let mut horizontal_vector: Vec<Vec<u8>> = Vec::new();
        for width_element in 0..width {
            let mut rgb_vector: Vec<u8> = Vec::new();
            let pixel = img.get_pixel(width_element, height_element);
            rgb_vector.push(pixel[0]);
            rgb_vector.push(pixel[1]);
            rgb_vector.push(pixel[2]);
            horizontal_vector.push(rgb_vector);
        }
        output.push(horizontal_vector);
    }

    return output;

}

fn rgb_to_intensity_vector(image_vector: Vec<Vec<Vec<u8>>>) -> Vec<Vec<u8>> {
    let mut output: Vec<Vec<u8>> = Vec::new();
    for horizontal_vector in image_vector {
        let mut horizontal_intensity_vector: Vec<u8> = Vec::new();
        for rgb_vector in horizontal_vector {
            let value = (rgb_vector[0] + rgb_vector[1] + rgb_vector[2]) / 3;
            horizontal_intensity_vector.push(value);
        }
        output.push(horizontal_intensity_vector);
    }
    return output;
}

fn characters_vector(intensity_vector: Vec<Vec<u8>>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    for horizontal_vector in intensity_vector {
        let mut char_horizontal_vector: Vec<char> = Vec::new();
        for pixel_value in horizontal_vector {
            let ascii_payload = String::from("-@%*+/:;. ");
            let mut density: usize = (pixel_value as usize) * (ascii_payload.len() as usize) / 255;
            if density == ascii_payload.len() as usize {
                density -= 1;
            }
            match ascii_payload.chars().nth(density) {
                Some(ascii_char) => char_horizontal_vector.push(ascii_char),
                None => char_horizontal_vector.push(' '),
            }
        }
        output.push(char_horizontal_vector);
    }
    return output;
}

fn main() {
    
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("[*] Path to image not provided!");
        std::process::exit(1);
    }

    let image_vector: Vec<Vec<Vec<u8>>> = image_to_vector(args[0].clone()); 
    let intensity_vector: Vec<Vec<u8>> = rgb_to_intensity_vector(image_vector);
    let character_image_vector: Vec<Vec<char>> = characters_vector(intensity_vector);

    for horizontal_vector in character_image_vector {
        for pixel in horizontal_vector {
            print!("{}  ", pixel);
        }
        print!("\n");
    }
}
