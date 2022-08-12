#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use may::win32::*;
use core::ptr::null_mut;

const ID_TEST: u32 = 13; 

fn main() {
    let instance = get_process_handle();

    let mut wc = WNDCLASSW::default();
    wc.lpsz_class_name = wide_null("master").as_ptr();
    wc.lpfn_wnd_proc = Some(window_procedure);
    wc.h_instance = instance;
    wc.h_cursor = load_predefined_cursor(IDCursor::Arrow).unwrap();
    wc.style = CS_HREDRAW | CS_VREDRAW;

    let _atom = register_class(&wc).unwrap();

    let hwnd = create_app_window("master", "spotify", 600, 400, instance).unwrap();

    let mut data = (0.1_f32, 1_i32);
    let data_ptr: Handle = &mut data as *mut _ as Handle;
    let test_hwnd = create_control_window("button", 40, 40, hwnd, ID_TEST, instance).unwrap();
    set_window_subclass(test_hwnd, Some(test_procedure), ID_TEST as usize).unwrap();
    set_prop(test_hwnd, "test", &mut data).unwrap();

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
            return 0;
        }
        WM_SIZE => return 0,
        WM_DESTROY => {
            post_quit_message(0);
            return 0;
        }
        WM_CLOSE => {
            drop(DestroyWindow(hwnd));
            return 0;
        }
        WM_ERASEBKGND => return 0,
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

pub unsafe extern "system" fn test_procedure(
    hwnd: Hwnd,
    msg: Uint,
    wparam: Wparam,
    lparam: Lparam,
    uidsubclass: UintPtr,
    dwrefdata: DwordPtr,
) -> Lresult {
    match msg {
        0x0111 => {
            let data: &mut (f32, i32) = get_prop(hwnd, "test").unwrap();
            println!("{:?}", data);
            return 0;
        }
        WM_NCDESTROY => {
            drop(RemovePropW(hwnd, wide_null("test").as_ptr()));
            return 0;
        }
        _ => return DefSubclassProc(hwnd, msg, wparam, lparam),
    }
}
