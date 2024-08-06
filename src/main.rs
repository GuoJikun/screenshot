use std::{io::ErrorKind::WouldBlock, path::Path, thread, time::Duration};
use image::{ImageBuffer, Rgba};

#[derive(Debug)]
pub struct Screenshot {
    image: ImageBuffer::<Rgba<u8>, Vec<u8>>
}

#[cfg(target_os = "windows")]
use scrap::{Capturer as WindowsCapturer, Display as WindowsDisplay};

#[cfg(target_os = "linux")]
use scrap::{Capturer as LinuxCapturer, Display as LinuxDisplay};

#[cfg(target_os = "macos")]
use coregraphics::display::CGDisplay;

impl Screenshot {
    #[cfg(target_os = "windows")]
    pub fn new() -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
        let display = WindowsDisplay::primary().map_err(|e| e.to_string())?;
        let (w, h) = (display.width(), display.height());
        let mut capturer = WindowsCapturer::new(display).map_err(|e| e.to_string())?;

        loop {
            match capturer.frame() {
                Ok(frame) => {
                    // 将 BGR 转换为 RGB
                    let buffer: Vec<u8> = frame
                        .chunks(4)
                        .flat_map(|pixel| vec![pixel[2], pixel[1], pixel[0], pixel[3]])
                        .collect();

                    let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(w as u32, h as u32, buffer)
                        .ok_or("Failed to create image buffer.")?;
                    
                    Screenshot {
                        image: img.clone()
                    };

                    return Ok(img);
                }
                Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    } else {
                        return Err(error.to_string());
                    }
                }
            }
        }
    }
    #[cfg(target_os = "linux")]
    pub fn new() -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
        let display = LinuxDisplay::primary().map_err(|e| e.to_string())?;
        let (w, h) = (display.width(), display.height());
        let mut capturer = LinuxCapturer::new(display).map_err(|e| e.to_string())?;

        loop {
            match capturer.frame() {
                Ok(frame) => {
                    // 将 BGR 转换为 RGB
                    let buffer: Vec<u8> = frame
                        .chunks(4)
                        .flat_map(|pixel| vec![pixel[2], pixel[1], pixel[0], pixel[3]])
                        .collect();
                    let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(w as u32, h as u32, buffer)
                        .ok_or("Failed to create image buffer.")?;
                    Screenshot {
                        image: img.clone()
                    };
                    return Ok(img);
                }
                Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    } else {
                        return Err(error.to_string());
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub fn new() -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
        let display_id = CGDisplay::main().id();
        let image = CGDisplay::main().image().ok_or("Failed to capture screenshot")?;
        let width = image.width() as u32;
        let height = image.height() as u32;

        let mut buffer: Vec<u8> = vec![0; (width * height * 4) as usize];
        image.data().copy_into_slice(&mut buffer);

        let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, buffer)
            .ok_or("Failed to create image buffer.")?;
        Screenshot {
            image: img.clone()
        };
        return Ok(img);
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        self.image.save(path).map_err(|e| e.to_string())
    }

} 



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