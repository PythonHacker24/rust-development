extern crate image; 

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
            if pixel_value < 50 { char_horizontal_vector.push('#') }
            else if pixel_value >=80 && pixel_value < 100 { char_horizontal_vector.push('$') }
            else if pixel_value >=100 && pixel_value < 120 { char_horizontal_vector.push('*') }
            else if pixel_value >= 120 && pixel_value < 140 { char_horizontal_vector.push('-') }
            else if pixel_value >=140 && pixel_value < 160 { char_horizontal_vector.push('=') }
            else if pixel_value >= 160 && pixel_value < 180 { char_horizontal_vector.push(';')}
            else if pixel_value >= 180 && pixel_value < 220 { char_horizontal_vector.push(';') }
            else if pixel_value >= 220 && pixel_value < 240 { char_horizontal_vector.push(',') }
            else { char_horizontal_vector.push('.') } 
        }
        output.push(char_horizontal_vector);
    }
    return output;
}

fn main() {
   
    let image_vector: Vec<Vec<Vec<u8>>> = image_to_vector("/Users/adityapatil/Downloads/cute_small_girl.jpeg".to_string()); 
    let intensity_vector: Vec<Vec<u8>> = rgb_to_intensity_vector(image_vector);
    let character_image_vector: Vec<Vec<char>> = characters_vector(intensity_vector);

    for horizontal_vector in character_image_vector {
        for pixel in horizontal_vector {
            print!("{}  ", pixel);
        }
        print!("\n");
    }
}
