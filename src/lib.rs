#[cfg(windows)]
pub mod win32;

use win32::{HICON, HWND, create_custom_button, set_window_userdata, load_icon, Win32Error};

pub trait Position {
    fn get_position(&mut self) -> (&mut (f32, i32), &mut (f32, i32));
}

pub struct CustomBtn {
    pub hwnd: HWND,
    pub icons: Vec<HICON>,
    pub index: usize,
    pub x: (f32, i32),
    pub y: (f32, i32),
}
impl CustomBtn {
    pub fn new(parent: HWND, x: (f32, i32), y: (f32, i32), width: i32, height: i32, filepath: Vec<&str>) -> Result<CustomBtn, Win32Error> {
        let hwnd = create_custom_button(width, height, parent)?;
        let custom_btn = CustomBtn {
            hwnd,
            icons: {
                let mut vec = Vec::new();
                for path in filepath.iter() {
                    vec.push(load_icon(path)?);
                }
                vec
            },
            index: 0,
            x,
            y,
        };
        Ok(custom_btn)
    }
    pub fn set_userdata(&mut self) -> Result<(), Win32Error> {
        let _ = set_window_userdata::<CustomBtn>(self.hwnd, self)?;
        Ok(())
    }
}
impl Position for CustomBtn {
    fn get_position(&mut self) -> (&mut (f32, i32), &mut (f32, i32)) {
        (&mut self.x, &mut self.y)
    }
}
