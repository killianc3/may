#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::ptr::null_mut;
use may::win32::*;

pub struct WindowData {
}
impl Default for WindowData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub struct ButtonData {
    pub texture: [HICON; 2],
    pub index: usize,
    pub x: (f32, f32),
    pub y: (f32, f32),
}
impl Default for ButtonData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

fn main() {
    let instance = get_process_handle();
    
    let sample_window_class = "Sample Window Class";
    let sample_window_class_wn = wide_null(sample_window_class);

    let mut wc = WNDCLASSW::default();
    wc.lpfnWndProc = Some(window_procedure);
    wc.hInstance = instance;
    wc.hIcon = load_icon("spotify.ico").unwrap();
    wc.lpszClassName = sample_window_class_wn.as_ptr();
    wc.hCursor = load_predefined_cursor(IDCursor::Arrow).unwrap();
    wc.style = CS_HREDRAW | CS_VREDRAW;

    let _atom = unsafe { register_class(&wc) }.unwrap();

    let hwnd = unsafe {
        create_app_window(
            sample_window_class,
            "Spotify",
            None,
            [600, 400],
            null_mut(),
            )
    }.unwrap();

    let btn = unsafe {
        create_custom_button(
            [0, 0],
            [32, 32],
            hwnd,
            null_mut(),
            )
    }.unwrap();

    let btn_hrgn = create_round_rect_rgn(0, 0, 32, 32, 32, 32).unwrap();
    unsafe { SetWindowRgn(btn, btn_hrgn, 1) };

    let mut ptr = ButtonData { 
        texture: [load_icon("btn.ico").unwrap(), load_icon("btn2.ico").unwrap()],
        index: 0,
        x: (0.5, 0.0),
        y: (1.0, -74.0),
    };

    if let Err(e) = unsafe { set_window_userdata::<ButtonData>(btn, &mut ptr) } {
        panic!("Error when set window userdata: {}", e);
    };

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

    loop {
        match get_any_message() {
            Ok(msg) => {
                if msg.message == WM_QUIT {
                    std::process::exit(msg.wParam as i32);
                }
                translate_message(&msg);
                unsafe {
                    DispatchMessageW(&msg);
                }
            }
            Err(e) => panic!("Error when getting from the message queue: {}", e),
        }
    }
}

pub unsafe extern "system" fn window_procedure(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => drop(DestroyWindow(hwnd)),
        WM_NCCREATE => {
            let createstruct: *mut CREATESTRUCTW = lparam as *mut _;
            if createstruct.is_null() {
                eprintln!("createstruct pointer was null");
                return 0;
            }
            let ptr = (*createstruct).lpCreateParams as *mut WindowData;
            if let Err(e) = set_window_userdata::<WindowData>(hwnd, ptr) {
                println!("Couldn't set the WindowData pointer: {}", e);
                return 0;
            }
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
        WM_DESTROY => {
            match get_window_userdata::<WindowData>(hwnd) {
                Ok(ptr) if !ptr.is_null() => {
                    Box::from_raw(ptr);
                    println!("Cleaned up the box.");
                }
                Ok(_) => {
                    println!("userdata ptr is null, no cleanup")
                }
                Err(e) => {
                    println!("Error while getting the userdata ptr to clean it up: {}", e)
                }
            }
            post_quit_message(0);
        }
        WM_PAINT => {
            match begin_paint(hwnd) {
                Ok((_hdc, ps)) => {
                    end_paint(hwnd, &ps);
                }
                Err(e) => {
                    println!("Couldn't begin painting: {}", e)
                }
            }
        }
        WM_DRAWITEM => {
            let draw_st = lparam as *mut DRAWITEMSTRUCT;
            match get_window_userdata::<ButtonData>((*draw_st).hwndItem) {
                Ok(ptr) if !ptr.is_null() => {
                    let rc = &(*draw_st).rcItem;
                    match draw_icon_ex((*draw_st).hDC, 0, 0, (*ptr).texture[(*ptr).index], rc.right - rc.left, 
                                       rc.bottom - rc.top, 0, null_mut(), DI_NORMAL) {
                        Ok(_) => {
                            if (*draw_st).itemState == 17 {
                                if (*ptr).index == 0 {
                                    (*ptr).index = 1
                                } else {
                                    (*ptr).index = 0
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error while drawing an icon: {}", e)
                        }
                    }
                }
                Ok(_) => {
                    println!("userdata ptr is null")
                }
                Err(e) => {
                    println!("Error while getting the userdata ptr: {}", e)
                }
            }
            return 1;
        }
        WM_SIZE => {
            let dims = (LOWORD(lparam as u32), HIWORD(lparam as u32));
            EnumChildWindows(hwnd, Some(enum_procedure), &dims as *const _ as LPARAM);
            return 0
        }
        WM_ERASEBKGND => {
            let rc = RECT::default();
            if let Err(e) = get_client_rect(hwnd, &rc) {
                println!("Error while getting client rect: {}", e);
            }
            let side_rc = RECT { left: 0, top: 0, right: 240, bottom: rc.bottom - 92 };
            let _ = fill_rect(wparam as HDC, &side_rc, CreateSolidBrush(SIDE_COLORREF)).unwrap();
            let play_rc = RECT { left: 0, top: rc.bottom - 91, right: rc.right, bottom: rc.bottom };
            let _ = fill_rect(wparam as HDC, &play_rc, CreateSolidBrush(PLAY_COLORREF)).unwrap();
            let playtop_rc = RECT { left: 0, top: rc.bottom - 92, right: rc.right, bottom: rc.bottom - 91 };
            let _ = fill_rect(wparam as HDC, &playtop_rc, CreateSolidBrush(PLAYTOP_COLORREF)).unwrap();
            return 1
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    0
}

pub unsafe extern "system" fn enum_procedure(child: HWND, lparam: LPARAM) -> BOOL {
    match get_window_userdata::<ButtonData>(child) {
        Ok(ptr) if !ptr.is_null() => {
            let dims = lparam as *const (u16, u16);
            let x = (*dims).0 as f32 * (*ptr).x.0 + (*ptr).x.1;
            let y = (*dims).1 as f32 * (*ptr).y.0 + (*ptr).y.1;

            if let Err(e) = move_window(child, x as i32, y as i32, 32, 32, 1) {
                println!("Error while moving a window: {}", e);
                return 0
            }
        }
        Ok(_) => {
            println!("userdata ptr is null")
        }
        Err(e) => {
            println!("Error while getting the userdata ptr: {}", e);
            return 0
        }
    }
    return 1
}
