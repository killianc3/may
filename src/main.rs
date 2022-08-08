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
    pub texture: Vec<HICON>,
    pub index: usize,
    pub x: (f32, f32),
    pub y: (f32, f32),
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

    let _atom = register_class(&wc).unwrap();

    let hwnd = create_app_window(
        sample_window_class,
        "Spotify",
        None,
        [600, 400],
        null_mut(),
        ).unwrap();

    let btn_play = create_custom_button(32, 32, hwnd).unwrap();
    let btn_forward = create_custom_button(32, 32, hwnd).unwrap();
    let btn_backward = create_custom_button(32, 32, hwnd).unwrap();
    let btn_shuffle = create_custom_button(32, 32, hwnd).unwrap();
    let btn_repeat = create_custom_button(32, 32, hwnd).unwrap();
    let btn_like = create_custom_button(32, 32, hwnd).unwrap();
    let btn_mike = create_custom_button(32, 32, hwnd).unwrap();
    let btn_list = create_custom_button(32, 32, hwnd).unwrap();

    let btn_play_hrgn = create_round_rect_rgn(0, 0, 32, 32, 32, 32).unwrap();
    let _ = set_window_rgn(btn_play, btn_play_hrgn, 1).unwrap();

    let mut btn_play_data = ButtonData { 
        texture: vec!(load_icon("icon/play.ico").unwrap(), load_icon("icon/play2.ico").unwrap()),
        index: 0,
        x: (0.5, -16.0),
        y: (1.0, -74.0),
    };

    let mut btn_forward_data = ButtonData {
        texture: vec!(load_icon("icon/forward.ico").unwrap()),
        index: 0,
        x: (0.5, 32.0),
        y: (1.0, -74.0),
    };

    let mut btn_backward_data = ButtonData {
        texture: vec!(load_icon("icon/backward.ico").unwrap()),
        index: 0,
        x: (0.5, -64.0),
        y: (1.0, -74.0),
    };

    let mut btn_shuffle_data = ButtonData {
        texture: vec!(load_icon("icon/shuffle.ico").unwrap(), load_icon("icon/shuffle2.ico").unwrap()),
        index: 0,
        x: (0.5, -104.0),
        y: (1.0, -74.0),
    };

    let mut btn_repeat_data = ButtonData {
        texture: vec!(load_icon("icon/repeat.ico").unwrap(), load_icon("icon/repeat2.ico").unwrap(), load_icon("icon/repeat3.ico").unwrap()),
        index: 0,
        x: (0.5, 72.0),
        y: (1.0, -74.0),
    };

    let mut btn_like_data = ButtonData {
        texture: vec!(load_icon("icon/like.ico").unwrap(), load_icon("icon/like2.ico").unwrap()),
        index: 0,
        x: (0.0, 169.0),
        y: (1.0, -61.0),
    };

    let mut btn_mike_data = ButtonData {
        texture: vec!(load_icon("icon/mike.ico").unwrap(), load_icon("icon/mike2.ico").unwrap()),
        index: 0,
        x: (1.0, -267.0),
        y: (1.0, -61.0),
    };

    let mut btn_list_data = ButtonData {
        texture: vec!(load_icon("icon/list.ico").unwrap(), load_icon("icon/list2.ico").unwrap()),
        index: 0,
        x: (1.0, -236.0),
        y: (1.0, -61.0),
    };


    let _ = set_window_userdata::<ButtonData>(btn_play, &mut btn_play_data).unwrap();
    let _ = set_window_userdata::<ButtonData>(btn_forward, &mut btn_forward_data).unwrap();
    let _ = set_window_userdata::<ButtonData>(btn_backward, &mut btn_backward_data).unwrap();
    let _ = set_window_userdata::<ButtonData>(btn_shuffle, &mut btn_shuffle_data).unwrap();
    let _ = set_window_userdata::<ButtonData>(btn_repeat, &mut btn_repeat_data).unwrap();
    let _ = set_window_userdata::<ButtonData>(btn_like, &mut btn_like_data).unwrap();
    let _ = set_window_userdata::<ButtonData>(btn_mike, &mut btn_mike_data).unwrap();
    let _ = set_window_userdata::<ButtonData>(btn_list, &mut btn_list_data).unwrap();

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
            println!("{}", (*draw_st).itemState);
            match get_window_userdata::<ButtonData>((*draw_st).hwndItem) {
                Ok(ptr) if !ptr.is_null() => {
                    let rc = &(*draw_st).rcItem;
                    match draw_icon_ex((*draw_st).hDC, 0, 0, (*ptr).texture[(*ptr).index], rc.right - rc.left, 
                                       rc.bottom - rc.top, 0, null_mut(), DI_NORMAL) {
                        Ok(_) => {
                            if (*draw_st).itemState == 17 {
                                if (*ptr).index == (*ptr).texture.len() - 1 {
                                    (*ptr).index = 0
                                } else {
                                    (*ptr).index += 1
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
        0x02A1 => println!("something append"),
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
