#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use may::win32::*;
use may::Btn;

fn main() {
    let instance = get_process_handle();

    let mut wc = WNDCLASSW::default();
    wc.lpsz_class_name = wide_null("master").as_ptr();
    wc.lpfn_wnd_proc = Some(window_procedure);
    wc.h_instance = instance;
    wc.h_cursor = load_predefined_cursor(IDCursor::Arrow).unwrap();
    wc.style = CS_HREDRAW | CS_VREDRAW;

    let _atom = register_class(&wc).unwrap();

    let mut blue_brush = create_brush(0, 0, 255).unwrap();
    let hwnd = create_app_window("master", "spotify", 600, 400, instance).unwrap();
    set_prop(hwnd, "blue_brush", &mut blue_brush).unwrap();

    let mut btn = Btn::new(instance, (0.5, 0), (1.0, -74), vec![("btn.ico", "btn_h.ico")]);
    let child = btn.init(hwnd, instance, 32, 32).unwrap();

    //let mut ressource = build_ressource(instance).unwrap();
    //let mut hwnds = build_window(&mut ressource, hwnd, instance).unwrap();

    //set_prop(hwnd, "hwnds", &mut hwnds).unwrap();

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
                Ok((hdc, ps)) => {
                    let blue_brush: &mut Hbrush = get_prop(hwnd, "blue_brush").unwrap();
                    fill_rect(hdc, &ps.rc_paint, *blue_brush).unwrap();
                    end_paint(hwnd, &ps);
                }
                Err(e) => println!("Couldn't begin painting: {}", e),
            }
            return 0;
        }
        WM_SIZE => {
            /*
            let width = loword(lparam as u32);
            let height = hiword(lparam as u32);
            let hwnds: &mut Vec<Hwnd> = get_prop(hwnd, "hwnds").unwrap();
            let mut hdwp = begin_defer_window_pos(hwnds.len() as i32).unwrap();
            for child in &*hwnds {
                let pos: &mut ((f32, i32), (f32, i32)) = get_prop(*child, "pos").unwrap();
                let x = (pos.0.0 * width as f32) as i32 + pos.0.1;
                let y = (pos.1.0 * height as f32) as i32 + pos.1.1;
                hdwp = DeferWindowPos(hdwp, *child, null_mut(), x, y, 0, 0, 0x0001 as u32);
            }
            end_defer_window_pos(hdwp).unwrap();
            */
            return 0;
        }
        WM_DESTROY => {
            post_quit_message(0);
            return 0;
        }
        WM_CLOSE => {
            drop(DestroyWindow(hwnd));
            drop(RemovePropW(hwnd, wide_null("blue_brush").as_ptr()));
            //drop(RemovePropW(hwnd, wide_null("hwnds").as_ptr()));
            return 0;
        }
        WM_ERASEBKGND => return 0,
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
