#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::ptr::null_mut;
use may::win32::*;

fn main() {
    let instance = get_process_handle();

    let mut wc = WNDCLASSW::default();
    wc.lpsz_class_name = wide_null("master").as_ptr();
    wc.lpfn_wnd_proc = Some(window_procedure);
    wc.h_instance = instance;
    wc.h_cursor = load_predefined_cursor(IDCursor::Arrow).unwrap();
    wc.style = CS_HREDRAW | CS_VREDRAW;

    let _atom = register_class(&wc).unwrap();

    let hwnd = unsafe { CreateWindowExW(
            0,
            wide_null("master").as_ptr(),
            wide_null("spotify").as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            600,
            400,
            null_mut(),
            null_mut(),
            instance,
            null_mut(),
            )
    };

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

    loop {
        match get_any_message() {
            Ok(msg) => {
                if msg.message == WM_QUIT {
                    std::process::exit(msg.w_param as i32);
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

pub unsafe extern "system" fn window_procedure(
    hwnd: Hwnd,
    msg: Uint,
    wparam: Wparam,
    lparam: Lparam,
) -> Lresult {
    match msg {
        WM_CREATE => return 0,
        WM_PAINT => {
            match begin_paint(hwnd) {
                Ok((_hdc, ps)) => end_paint(hwnd, &ps),
                Err(e) => println!("Couldn't begin painting: {}", e),
            }
            return 0
        }
        WM_SIZE => return 0,
        WM_DESTROY => {
            post_quit_message(0);
            return 0
        }
        WM_CLOSE => {
            drop(DestroyWindow(hwnd));
            return 0
        }
        WM_ERASEBKGND => return 0,
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
