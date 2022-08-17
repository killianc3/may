use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::UI::Controls::*,
    Win32::UI::Shell::DefSubclassProc, Win32::UI::WindowsAndMessaging::*,
};

pub fn create_window<T: Into<PCWSTR>>(
    class: T,
    name: &str,
    [width, height]: [i32; 2],
    instance: HINSTANCE,
) -> HWND {
    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class,
            PCWSTR::from(&HSTRING::from(name)),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            None,
            None,
            instance,
            std::ptr::null(),
        )
    }
}

pub fn create_control<T: Into<PCWSTR>>(
    class: T,
    [width, height]: [i32; 2],
    parent: HWND,
    instance: HINSTANCE,
) -> HWND {
    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class,
            PCWSTR::null(),
            WS_CHILD | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            parent,
            None,
            instance,
            std::ptr::null(),
        )
    }
}

enum State {
    Idle,
    Down,
    Hover,
}

pub struct ButtonData {
    state: State,
    index: usize,
    icons: Vec<[HICON; 2]>,
}
impl ButtonData {
    pub unsafe fn new(icons_file: Vec<[&str; 2]>, instance: HINSTANCE) -> Result<ButtonData> {
        Ok(ButtonData {
            state: State::Idle,
            index: 0,
            icons: {
                let mut icons = Vec::new();
                for [idle, hover] in icons_file {
                    let idle_ico = HICON(
                        LoadImageW(
                            instance,
                            PCWSTR::from(&HSTRING::from(idle)),
                            IMAGE_ICON,
                            0,
                            0,
                            LR_LOADFROMFILE,
                        )?
                        .0,
                    );
                    let hover_ico = HICON(
                        LoadImageW(
                            instance,
                            PCWSTR::from(&HSTRING::from(hover)),
                            IMAGE_ICON,
                            0,
                            0,
                            LR_LOADFROMFILE,
                        )?
                        .0,
                    );
                    icons.push([idle_ico, hover_ico]);
                }
                icons
            },
        })
    }
}

pub extern "system" fn buttonproc(
    hwnd: HWND,
    umsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    _uidsubclass: usize,
    dwrefdata: usize,
) -> LRESULT {
    unsafe {
        match umsg as u32 {
            WM_PAINT => {
                let data = dwrefdata as *mut ButtonData;
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);
                match (*data).state {
                    State::Down => {
                        if (*data).index == (*data).icons.len() - 1 {
                            (*data).index = 0
                        } else {
                            (*data).index += 1
                        }
                        DrawIcon(hdc, 0, 0, (*data).icons[(*data).index][0])
                            .ok()
                            .unwrap();
                    }
                    State::Hover => DrawIcon(hdc, 0, 0, (*data).icons[(*data).index][1])
                        .ok()
                        .unwrap(),
                    State::Idle => DrawIcon(hdc, 0, 0, (*data).icons[(*data).index][0])
                        .ok()
                        .unwrap(),
                }
                EndPaint(hwnd, &mut ps).ok().unwrap();
                return LRESULT(0);
            }
            WM_DESTROY => {
                return LRESULT(0);
            }
            WM_MOUSEMOVE => {
                let data = dwrefdata as *mut ButtonData;
                (*data).state = State::Hover;
            }
            WM_LBUTTONDOWN => {
                let data = dwrefdata as *mut ButtonData;
                (*data).state = State::Down;
            }
            WM_LBUTTONUP => {
                let data = dwrefdata as *mut ButtonData;
                (*data).state = State::Hover;
            }
            WM_MOUSELEAVE => {
                let data = dwrefdata as *mut ButtonData;
                (*data).state = State::Idle;
            }
            _ => (),
        }
        DefSubclassProc(hwnd, umsg, wparam, lparam)
    }
}

pub fn loword(l: u32) -> u16 {
    (l & 0xffff) as u16
}
pub fn hiword(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}
