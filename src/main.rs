mod screen;
use screen::{capture_screen, get_screen_res};

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
            let filename = format!("capture_{}.png", timestamp);

            cap.save(&filename).unwrap();
            println!("Screen was captured and saved as: {}", filename);
        }
        Err(e) => eprintln!("Failed to capture screen: {}", e),
    }
}
