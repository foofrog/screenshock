mod screen;
use std::fs::File;
use std::io::BufWriter;

use image::{
    buffer::ConvertBuffer,
    codecs::gif::{GifEncoder, Repeat},
    Delay, Frame,
};
use screen::{capture_num_screens, capture_screen, get_screen_res};

use chrono::Utc;

fn main() {
    match get_screen_res() {
        Ok(screen_res) => println!(
            "Screen resolution: {}x{}",
            screen_res.width, screen_res.height
        ),
        Err(e) => eprintln!("Failed to get screen resolution: {}", e),
    }

    match capture_screen() {
        Ok(cap) => {
            let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
            let filename = format!("capture_s_{}.png", timestamp);

            cap.save(&filename).unwrap();
            println!("Screen was captured and saved as: {}", filename);
        }
        Err(e) => eprintln!("Failed to capture screen: {}", e),
    }

    match capture_num_screens(40) {
        Ok(caps) => {
            let frames: Vec<Frame> = caps
                .into_iter()
                .map(|frame| {
                    let buffer = frame.convert();
                    let delay = Delay::from_numer_denom_ms(50, 100);
                    let iframe = Frame::from_parts(buffer, 0, 0, delay);
                    println!("Frame dimensions: {:?}", frame.dimensions());
                    println!("Frame delay: {:?}", delay);
                    iframe
                })
                .collect();

            let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
            let filename = format!("capture_d_{}.gif", timestamp);
            let file = File::create(&filename).unwrap();
            let writer = BufWriter::new(file);

            let mut encoder = GifEncoder::new(writer);
            encoder.set_repeat(Repeat::Infinite).unwrap();
            encoder.encode_frames(frames).unwrap();

            println!("Screens were captured and saved as: {}", filename)
        }
        Err(e) => eprintln!("Failed to capture screens: {}", e),
    }
}
