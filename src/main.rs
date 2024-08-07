pub mod platform;

use std::path::Path;

pub use platform::Screenshot;



fn main() {
    let _ = match Screenshot::new() {
        Ok(screenshot) => {
            screenshot.save(&Path::new("screenshot.png")).map_err(|e| e.to_string())
        },
        Err(_) => {
            Err("Failed to capture screenshot".to_string())
        },
    };
}