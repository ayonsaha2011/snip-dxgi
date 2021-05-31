use crate::{Screenshot, ScreenResult};
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::error::Error;

/// Get a screenshot of the requested display.
pub fn screen_capture(screen: usize) -> ScreenResult {
    println!("call screen_capture_dxgi");
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = match Display::primary() {
        Ok(disp) => disp,
        Err(err) => return Err("Couldn't find primary display.")
    };
    let mut capturer = match Capturer::new(display) {
        Ok(capturer) => capturer,
        Err(err) => return Err("Couldn't begin capture.")
    };
    let (w, h) = (capturer.width(), capturer.height());
    loop {
        // Wait until there's a frame.
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };


        // Flip the ARGB image into a BGRA image.
        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255,
                ]);
            }
        }

        let pixel_width: usize = 4; // FIXME

        break Ok(Screenshot {
            data: bitflipped,
            height: h as usize,
            width: w as usize,
            row_len: w as usize * pixel_width,
            pixel_width,
        })
    }
}