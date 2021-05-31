//! Capture a Screen of a display. The resulting screenshot is stored in
//! the `Screenshot` struct, which varies per platform.
extern crate scrap;
// #![allow(unused_assignments)]
#[cfg(windows)]
#[path = "win/mod.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod platform;

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod platform;

pub use platform::screen_capture;
use std::mem::size_of;

pub type ScreenResult = Result<Screenshot, &'static str>;

#[derive(Clone, Copy)]
pub struct Pixel {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// An image buffer containing the screenshot.
/// Pixels are stored as [ARGB](https://en.wikipedia.org/wiki/ARGB).
pub struct Screenshot {
    data: Vec<u8>,
    height: usize,
    width: usize,
    row_len: usize,
    // Might be superfluous
    pixel_width: usize,
}

impl Screenshot {
    /// Height of image in pixels.
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Width of image in pixels.
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Number of bytes in one row of bitmap.
    #[inline]
    pub fn row_len(&self) -> usize {
        self.row_len
    }

    /// Width of pixel in bytes.
    #[inline]
    pub fn pixel_width(&self) -> usize {
        self.pixel_width
    }

    /// Bitmap as vector.
    #[inline]
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Raw bitmap.
    /// # Safety
    #[inline]
    pub unsafe fn raw_data(&self) -> *const u8 {
        &self.data[0] as *const u8
    }

    /// Raw bitmap.
    /// # Safety
    #[inline]
    pub unsafe fn raw_data_mut(&mut self) -> *mut u8 {
        &mut self.data[0] as *mut u8
    }

    /// Number of bytes in bitmap
    #[inline]
    pub fn raw_len(&self) -> usize {
        self.data.len() * size_of::<u8>()
    }

    /// Gets pixel at (row, col)
    pub fn get_pixel(&self, row: usize, col: usize) -> Pixel {
        let idx = row * self.row_len() + col * self.pixel_width();
        unsafe {
            //let data = &self.data[0] as *const u8;
            if idx > self.data.len() {
                panic!("Bounds overflow");
            }

            Pixel {
                a: *self.data.get_unchecked(idx + 3),
                r: *self.data.get_unchecked(idx + 2),
                g: *self.data.get_unchecked(idx + 1),
                b: *self.data.get_unchecked(idx),
            }
        }
    }
}

impl AsRef<[u8]> for Screenshot {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.data.as_slice()
    }
}

#[test]
fn test_screen_capture() {
    let s: Screenshot = screen_capture(0).unwrap();
    println!(
        "width: {}\n height: {}\npixel width: {}\n bytes: {}",
        s.width(),
        s.height(),
        s.pixel_width(),
        s.raw_len()
    );
}
