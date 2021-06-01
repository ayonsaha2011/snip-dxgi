use crate::{ScreenResult, Screenshot};
use std::mem::size_of;
use winapi::{
    shared::{minwindef, ntdef, windef},
    um::{shellscalingapi, wingdi, winuser},
};

/// Reorder rows in bitmap, last to first.
/// TODO rewrite functionally
fn flip_rows(data: Vec<u8>, height: usize, row_len: usize) -> Vec<u8> {
    let mut new_data = Vec::with_capacity(data.len());
    unsafe { new_data.set_len(data.len()) };
    for row_i in 0..height {
        for byte_i in 0..row_len {
            let old_idx = (height - row_i - 1) * row_len + byte_i;
            let new_idx = row_i * row_len + byte_i;
            new_data[new_idx] = data[old_idx];
        }
    }
    new_data
}

/// Reorder bytes in one pixel, BGR to RGB.
fn flip_rgb(mut data: Vec<u8>, height: usize, width: usize, pixel_width: usize) -> Vec<u8> {
    // FIXME support other pixel_width but when?
    assert_eq!(pixel_width, 4);
    let row_len = width * pixel_width;
    for y in 0..height {
        for x in 0..width {
            data.swap(
                y * row_len + x * pixel_width,
                y * row_len + x * pixel_width + 2,
            );
        }
    }
    data
}

/// Get a screenshot of the requested display.
pub fn screen_capture_gdi(screen: usize) -> ScreenResult {
    use std::ptr::null_mut;
    unsafe {
        let mut dpi_awareness: shellscalingapi::PROCESS_DPI_AWARENESS = 0;
        shellscalingapi::GetProcessDpiAwareness(null_mut(), &mut dpi_awareness);
        shellscalingapi::SetProcessDpiAwareness(shellscalingapi::PROCESS_PER_MONITOR_DPI_AWARE);

        let monitor = enumerate_monitors()
            .into_iter()
            .nth(screen)
            .ok_or("failed to get specified screen")?;
        // Enumerate monitors, getting a handle and DC for requested monitor.
        // loljk, because doing that on Windows is worse than death
        let h_dc_screen = winuser::GetDC(null_mut());

        // Create a Windows Bitmap, and copy the bits into it
        let h_dc = wingdi::CreateCompatibleDC(h_dc_screen);
        if h_dc.is_null() {
            return Err("Can't get a Windows display.");
        }

        let h_bmp = wingdi::CreateCompatibleBitmap(h_dc_screen, monitor.width, monitor.height);
        if h_bmp.is_null() {
            return Err("Can't create a Windows buffer");
        }

        let res = wingdi::SelectObject(h_dc, h_bmp as windef::HGDIOBJ);
        if res == ntdef::NULL || res == wingdi::HGDI_ERROR {
            return Err("Can't select Windows buffer.");
        }

        let res = wingdi::BitBlt(
            h_dc,
            0,
            0,
            monitor.width,
            monitor.height,
            h_dc_screen,
            monitor.left,
            monitor.top,
            wingdi::SRCCOPY | wingdi::CAPTUREBLT,
        );
        if res == 0 {
            return Err("Failed to copy screen to Windows buffer");
        }

        // Get image info
        let pixel_width: usize = 4; // FIXME

        let mut bmi = wingdi::BITMAPINFO {
            bmiHeader: wingdi::BITMAPINFOHEADER {
                biSize: size_of::<wingdi::BITMAPINFOHEADER>() as minwindef::DWORD,
                biWidth: monitor.width as ntdef::LONG,
                biHeight: monitor.height as ntdef::LONG,
                biPlanes: 1,
                biBitCount: 8 * pixel_width as minwindef::WORD,
                biCompression: wingdi::BI_RGB,
                biSizeImage: (monitor.width * monitor.height * pixel_width as minwindef::INT)
                    as minwindef::DWORD,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [wingdi::RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }],
        };

        // Create a Vec for image
        let size: usize = (monitor.width * monitor.height) as usize * pixel_width;
        let mut data: Vec<u8> = Vec::with_capacity(size);
        data.set_len(size);

        // copy bits into Vec
        wingdi::GetDIBits(
            h_dc,
            h_bmp,
            0,
            monitor.height as minwindef::DWORD,
            &mut data[0] as *mut u8 as minwindef::LPVOID,
            &mut bmi as wingdi::LPBITMAPINFO,
            wingdi::DIB_RGB_COLORS,
        );

        // Release native image buffers
        winuser::ReleaseDC(null_mut(), h_dc_screen); // don't need screen anymore
        wingdi::DeleteDC(h_dc);
        wingdi::DeleteObject(h_bmp as windef::HGDIOBJ);

        let data = flip_rows(
            data,
            monitor.height as usize,
            monitor.width as usize * pixel_width,
        );

        let data = flip_rgb(
            data,
            monitor.height as usize,
            monitor.width as usize,
            pixel_width as usize,
        );

        shellscalingapi::SetProcessDpiAwareness(dpi_awareness);

        Ok(Screenshot {
            data,
            height: monitor.height as usize,
            width: monitor.width as usize,
            row_len: monitor.width as usize * pixel_width,
            pixel_width,
        })
    }
}

struct Monitor {
    left: i32,
    top: i32,
    height: i32,
    width: i32,
}

fn enumerate_monitors() -> Vec<Monitor> {
    use std::mem::transmute;
    use std::ptr::{null, null_mut};

    extern "system" fn callback(
        _: windef::HMONITOR,
        _: windef::HDC,
        rect: windef::LPRECT,
        lparam: minwindef::LPARAM,
    ) -> minwindef::BOOL {
        let refresult = unsafe { transmute::<minwindef::LPARAM, *mut Vec<Monitor>>(lparam) };
        let refresult: &mut Vec<Monitor> = unsafe { &mut *refresult };

        let monitor = unsafe {
            Monitor {
                left: (*rect).left,
                top: (*rect).top,
                height: (*rect).bottom - (*rect).top,
                width: (*rect).right - (*rect).left,
            }
        };

        refresult.push(monitor);

        minwindef::TRUE
    }

    let mut result: Vec<Monitor> = Vec::new();

    unsafe {
        winuser::EnumDisplayMonitors(
            null_mut::<*mut u8>() as windef::HDC,
            null::<*const u8>() as windef::LPCRECT,
            Some(callback),
            transmute::<*mut Vec<Monitor>, minwindef::LPARAM>(&mut result),
        );
    }

    result
}
