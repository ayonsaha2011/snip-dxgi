use crate::ScreenResult;

mod gdi;
mod dxgi;


pub fn screen_capture(screen: usize) -> ScreenResult {
    match dxgi::screen_capture_dxgi(screen) {
        Ok(screen_shot) => Ok(screen_shot),
        Err(err) => {
            println!("{:?}", err);
            return gdi::screen_capture_gdi(screen)
        }
    }
}