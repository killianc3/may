use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::System::WindowsProgramming::*,
    Win32::UI::Controls::*, Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*,
};

pub fn create_window<T: Into<PCWSTR>>(
    class: T,
    name: &str,
    (width, height): (i32, i32),
    instance: HINSTANCE,
    lpparam: *const core::ffi::c_void,
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
            lpparam,
        )
    }
}

pub fn create_control<T: Into<PCWSTR>>(
    class: T,
    style: Option<u32>,
    (width, height): (i32, i32),
    parent: HWND,
    instance: HINSTANCE,
) -> HWND {
    let mut styles = WS_CHILD | WS_VISIBLE;
    if !style.is_none() {
        styles = styles | WINDOW_STYLE(style.unwrap());
    }
    unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class,
            PCWSTR::null(),
            styles,
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

#[derive(Default)]
pub enum State {
    #[default]
    Idle,
    Down,
    Hover,
}

#[derive(Default)]
pub struct Control {
    pub hwnd: Option<HWND>,
    pub class: HSTRING,
    pub style: Option<u32>,
    pub proc: SUBCLASSPROC,
    pub x: (f32, i32),
    pub y: (f32, i32),
    pub size: (i32, i32),
    pub data: usize,
}

#[derive(Default)]
pub struct ButtonData {
    pub state: State,
    pub index: usize,
    pub icons: Vec<[HICON; 2]>,
}

#[derive(Default)]
pub struct TrackbarData {
    pub state: State,
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
                let mut rc = RECT::default();
                GetClientRect(hwnd, &mut rc);
                let (width, height) = (rc.right - rc.left, rc.bottom - rc.top);
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
                        DrawIconEx(
                            hdc,
                            0,
                            0,
                            (*data).icons[(*data).index][0],
                            width,
                            height,
                            0,
                            HBRUSH(0),
                            DI_NORMAL,
                        )
                        .ok()
                        .unwrap();
                    }
                    State::Hover => DrawIconEx(
                        hdc,
                        0,
                        0,
                        (*data).icons[(*data).index][1],
                        width,
                        height,
                        0,
                        HBRUSH(0),
                        DI_NORMAL,
                    )
                    .ok()
                    .unwrap(),
                    State::Idle => DrawIconEx(
                        hdc,
                        0,
                        0,
                        (*data).icons[(*data).index][0],
                        width,
                        height,
                        0,
                        HBRUSH(0),
                        DI_NORMAL,
                    )
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

pub extern "system" fn trackbarproc(
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
                let mut client_rc = RECT::default();
                GetClientRect(hwnd, &mut client_rc);
                InvalidateRect(hwnd, &client_rc, BOOL(0)).ok().unwrap();

                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);

                let mut thumb = RECT::default();
                SendMessageW(
                    hwnd,
                    TBM_GETTHUMBRECT,
                    WPARAM(0),
                    LPARAM((&mut thumb) as *mut _ as isize),
                );

                let data = dwrefdata as *mut TrackbarData;

                SetDCBrushColor(hdc, 0x00181818);
                FillRect(hdc, &ps.rcPaint, HBRUSH(GetStockObject(DC_BRUSH).0));

                let old_brush = SelectObject(hdc, CreateSolidBrush(0x005E5E5E));
                let old_pen = SelectObject(hdc, CreatePen(PS_SOLID, 0, 0x005E5E5E));

                RoundRect(hdc, client_rc.left + 6, 6, client_rc.right - 6, 10, 2, 2);

                match (*data).state {
                    State::Idle => {
                        SelectObject(hdc, CreateSolidBrush(0x00FFFFFF));
                        SelectObject(hdc, CreatePen(PS_SOLID, 0, 0x00FFFFFF));
                        RoundRect(hdc, client_rc.left + 6, 6, thumb.right, 10, 2, 2);
                    }
                    State::Hover => {
                        SelectObject(hdc, CreateSolidBrush(0x001DB954));
                        SelectObject(hdc, CreatePen(PS_SOLID, 0, 0x001DB954));
                        RoundRect(hdc, client_rc.left + 6, 6, thumb.right, 10, 2, 2);
                        SelectObject(hdc, CreateSolidBrush(0x00FFFFFF));
                        SelectObject(hdc, CreatePen(PS_SOLID, 0, 0x00FFFFFF));
                        Ellipse(
                            hdc,
                            thumb.left - 2,
                            thumb.top + 1,
                            thumb.right + 2,
                            thumb.bottom - 1,
                        );
                    }
                    _ => (),
                }

                SelectObject(hdc, old_brush);
                SelectObject(hdc, old_pen);

                EndPaint(hwnd, &mut ps).ok().unwrap();
                LRESULT(0)
            }
            WM_MOUSEMOVE => {
                let data = dwrefdata as *mut TrackbarData;
                match (*data).state {
                    State::Idle => {
                        (*data).state = State::Hover;
                        let mut client_rc = RECT::default();
                        GetClientRect(hwnd, &mut client_rc);
                        InvalidateRect(hwnd, &client_rc, BOOL(0)).ok().unwrap();
                    }
                    _ => (),
                }
                DefSubclassProc(hwnd, umsg, wparam, lparam)
            }
            WM_MOUSELEAVE => {
                let data = dwrefdata as *mut TrackbarData;
                (*data).state = State::Idle;
                let mut client_rc = RECT::default();
                GetClientRect(hwnd, &mut client_rc);
                InvalidateRect(hwnd, &client_rc, BOOL(0)).ok().unwrap();
                DefSubclassProc(hwnd, umsg, wparam, lparam)
            }
            _ => DefSubclassProc(hwnd, umsg, wparam, lparam),
        }
    }
}

pub fn loword(l: u32) -> u16 {
    (l & 0xffff) as u16
}
pub fn hiword(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}

pub fn icon(path: &str, ins: HINSTANCE) -> Result<HICON> {
    unsafe {
        Ok(HICON(
            LoadImageW(
                ins,
                &HSTRING::from(format!("icons/{path}")),
                IMAGE_ICON,
                0,
                0,
                LR_LOADFROMFILE,
            )?
            .0,
        ))
    }
}

pub fn log_to_phy_rc(mut rc: RECT) -> RECT {
    unsafe {
        let dpi = GetDeviceCaps(GetDC(HWND(0)), LOGPIXELSX);
        rc.left = MulDiv(rc.left, dpi, 96);
        rc.top = MulDiv(rc.top, dpi, 96);
        rc.right = MulDiv(rc.right, dpi, 96);
        rc.bottom = MulDiv(rc.bottom, dpi, 96);
        rc
    }
}

pub fn phy_to_log_rc(mut rc: RECT) -> RECT {
    unsafe {
        let dpi = GetDeviceCaps(GetDC(HWND(0)), LOGPIXELSX);
        rc.left = MulDiv(rc.left, 96, dpi);
        rc.top = MulDiv(rc.top, 96, dpi);
        rc.right = MulDiv(rc.right, 96, dpi);
        rc.bottom = MulDiv(rc.bottom, 96, dpi);
        rc
    }
}

pub fn log_to_phy(x: i32, y: i32) -> (i32, i32) {
    unsafe {
        let dpi = GetDeviceCaps(GetDC(HWND(0)), LOGPIXELSX);
        (MulDiv(x, dpi, 96), MulDiv(y, dpi, 96))
    }
}

pub fn phy_to_log(x: i32, y: i32) -> (i32, i32) {
    unsafe {
        let dpi = GetDeviceCaps(GetDC(HWND(0)), LOGPIXELSX);
        (MulDiv(x, 96, dpi), MulDiv(y, 96, dpi))
    }
}
