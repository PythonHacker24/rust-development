extern crate ffmpeg_next as ffmpeg;

use ffmpeg::{format::input, media::Type, software::scaling::context::Context, util::frame::video::Video};
use std::{env, error::Error};
use image::{self, GenericImageView, Rgb, RgbImage};

// fn frame_extractor(video_path: String) -> Result<Vec<RgbImage>, Box<dyn Error>> {
//     ffmpeg::init()?;
//
//     let mut frames: Vec<RgbImage> = Vec::new();
//     let mut ictx = input(video_path)?.format_context()?;
//     ictx.configure(None, None)?;
//
//     let input_video_stream = ictx
//         .find_stream_by_type(Type::Video)
//         .ok_or("No video stream found")
//         .codec()
//         .decoder()
//         .video()
//         .ok_or("Video stream has no decoder")?;
//
//     let mut sws = Context::get(
//         input_video_stream.format(),
//         input_video_stream.width(),
//         input_video_stream.height(),
//         ffmpeg::util::frame::video::PixelFormat::RGB24,
//         input_video_stream.width(),
//         input_video_stream.height(),
//         ffmpeg::util::frame::video::PixelFormat::RGB24,
//         ffmpeg::software::scaling::flag::Point,
//         None,
//     )?;
//
//     for (i, (stream, packet)) in ictx.packet().enumerate() {
//         if stream.index() == input_video_stream.index() {
//             let mut frame = Video::eempty;
//             packet.decode_to(&mut frame)?;
//
//             let mut scaled_frame = Video::empty();
//             sws.run(&frame, &mut scaled_frame)?;
//
//             let image_data = scaled_frame.data(0);
//             let width = scaled_frame.width();
//             let height = scaled_frame.height();
//
//             let img = RgbImage::from_raw(width, height, image_data.to_vec()).ok_or("Image creation failed")?;
//
//             frames.push(img);
//         }
//     }
//
//     Ok(frames)
// }

fn frame_extractor(video_path: String) -> Result<Vec<RgbImage>, Box<dyn Error>> {
    ffmpeg::init()?;

    let mut frames: Vec<RgbImage> = Vec::new();

    // Open the video file
    let mut ictx = input(&video_path)?.format();

    // Find the video stream
    let input_video_stream = ictx
        .streams()
        .video()
        .next()
        .ok_or(Error::StreamNotFound)?;

    // Iterate over frames
    for (i, (stream, packet)) in ictx.packets() {
        if stream.id() == input_video_stream.index() {
            let mut frame = Video::empty();
            packet.decode_to(&mut frame, &stream)?;

            // Create an image from the frame
            let image_data = frame.as_rgb8().ok_or(Error::Other("Frame is not RGB"))?;
            let width = frame.width();
            let height = frame.height();

            let img = RgbImage::from_raw(width, height, image_data.to_vec()).ok_or(Error::Other("Image creation failed"))?;

            // Store the frame in the vector
            frames.push(img);
        }
    }

    Ok(frames)
} 

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
