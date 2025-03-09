use std::fs::File;
use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, Rgba};
use screenshots::Screen;

pub struct GameRecorder {
    frames: Vec<Vec<u8>>,
    screen: Screen,
    start_x: i32,
    start_y: i32,
    width: u32,
    height: u32,
}

impl GameRecorder {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        let screens = Screen::all().unwrap();
        Self {
            frames: Vec::new(),
            screen: screens[0].clone(),
            start_x: x+10,
            start_y: y,
            width,
            height,
        }
    }

    pub fn capture_frame(&mut self) {
        if let Ok(image) = self.screen.capture_area(self.start_x, self.start_y, self.width, self.height) {
            let buffer = image.into_raw();
            self.frames.push(buffer);
            // 限制帧数以避免内存占用过大
            if self.frames.len() > 150 { // 30秒 * 10帧/秒
                self.frames.remove(0);
            }
        }
    }

    pub fn save_gif(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let mut encoder = Encoder::new(file, self.width as u16, self.height as u16, &[])?;
        encoder.set_repeat(Repeat::Infinite)?;

        for frame_data in &self.frames {
            let rgba_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
                self.width,
                self.height,
                frame_data.clone(),
            ).unwrap();

            let mut frame = Frame::from_rgba(self.width as u16, self.height as u16, &mut rgba_image.into_raw());
            frame.delay = 10; // 100ms delay between frames (10fps)
            encoder.write_frame(&frame)?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.frames.clear();
    }
}