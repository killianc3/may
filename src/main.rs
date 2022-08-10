#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::ptr::null_mut;
use may::win32::*;
use may::CustomBtn;

pub struct WindowData {}

fn main() {
    let instance = get_process_handle();

    let sample_window_class = "Sample Window Class";
    let sample_window_class_wn = wide_null(sample_window_class);

    let mut wc = WNDCLASSW::default();
    wc.lpfnWndProc = Some(window_procedure);
    wc.hInstance = instance;
    wc.hIcon = load_icon("spotify").unwrap();
    wc.lpszClassName = sample_window_class_wn.as_ptr();
    wc.hCursor = load_predefined_cursor(IDCursor::Arrow).unwrap();
    wc.style = CS_HREDRAW | CS_VREDRAW;

    let _atom = register_class(&wc).unwrap();

    let hwnd = create_app_window(sample_window_class, "Spotify", None, [600, 400], null_mut()).unwrap();

    let trackbar = unsafe { CreateWindowExW(
            0,
            wide_null("msctls_trackbar32").as_ptr(),
            null_mut(),
            WS_CHILD | WS_VISIBLE,
            200,
            0,
            200,
            40,
            hwnd,
            2 as HMENU,
            null_mut(),
            null_mut(),
            )
    };

    let mut play = CustomBtn::new(hwnd, (0.5, -16), (1.0, -74), 32, 32, vec!["play", "play2"]).unwrap();
    play.set_userdata().unwrap();
    let mut backward = CustomBtn::new(hwnd, (0.5, -64), (1.0, -74), 32, 32, vec!["backward"]).unwrap();
    backward.set_userdata().unwrap();
    let mut forward = CustomBtn::new(hwnd, (0.5, 32), (1.0, -74), 32, 32, vec!["forward"]).unwrap();
    forward.set_userdata().unwrap();
    let mut shuffle = CustomBtn::new(hwnd, (0.5, -104), (1.0, -74), 32, 32, vec!["shuffle", "shuffle2"]).unwrap();
    shuffle.set_userdata().unwrap();
    let mut repeat = CustomBtn::new(hwnd, (0.5, 72), (1.0, -74), 32, 32, vec!["repeat", "repeat2", "repeat3"]).unwrap();
    repeat.set_userdata().unwrap();
    let mut like = CustomBtn::new(hwnd, (0.0, 147), (1.0, -61), 32, 32, vec!["like", "like2"]).unwrap();
    like.set_userdata().unwrap();
    let mut mike = CustomBtn::new(hwnd, (1.0, -267), (1.0, -61), 32, 32, vec!["mike", "mike2"]).unwrap();
    mike.set_userdata().unwrap();
    let mut list = CustomBtn::new(hwnd, (1.0, -236), (1.0, -61), 32, 32, vec!["list", "list2"]).unwrap();
    list.set_userdata().unwrap();
    let mut ban = CustomBtn::new(hwnd, (0.0, 179), (1.0, -61), 32, 32, vec!["ban"]).unwrap();
    ban.set_userdata().unwrap();
    let mut device = CustomBtn::new(hwnd, (1.0, -204), (1.0, -61), 32, 32, vec!["device"]).unwrap();
    device.set_userdata().unwrap();
    let mut volume = CustomBtn::new(hwnd, (1.0, -172), (1.0, -61), 32, 32, vec!["volume", "volume2"]).unwrap();
    volume.set_userdata().unwrap();
    let mut full = CustomBtn::new(hwnd, (1.0, -46), (1.0, -61), 32, 32, vec!["full"]).unwrap();
    full.set_userdata().unwrap();

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

pub unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
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
        WM_PAINT => match begin_paint(hwnd) {
            Ok((_hdc, ps)) => {
                end_paint(hwnd, &ps);
            }
            Err(e) => {
                println!("Couldn't begin painting: {}", e)
            }
        },
        WM_DRAWITEM => {
            let draw_st = lparam as *mut DRAWITEMSTRUCT;
            match (*draw_st).CtlType {
                ODT_BUTTON => match get_window_userdata::<CustomBtn>((*draw_st).hwndItem) {
                    Ok(data) => match draw_icon((*draw_st).hDC, 0, 0, (*data).icons[(*data).index])
                    {
                        Ok(_) => {
                            if (*draw_st).itemState == 17 {
                                if (*data).index == (*data).icons.len() - 1 {
                                    (*data).index = 0
                                } else {
                                    (*data).index += 1
                                }
                            }
                        }
                        Err(e) => println!("Error while drawing icon: {}", e),
                    },
                    Err(e) => println!("Error while getting userdata: {}", e),
                },
                _ => return 1,
            }
        }
        WM_SIZE => {
            let dims = (LOWORD(lparam as u32), HIWORD(lparam as u32));
            EnumChildWindows(hwnd, Some(enum_procedure), &dims as *const _ as LPARAM);
            return 0;
        }
        WM_ERASEBKGND => {
            let rc = RECT::default();
            if let Err(e) = get_client_rect(hwnd, &rc) {
                println!("Error while getting client rect: {}", e);
            }
            let side_rc = RECT {
                left: 0,
                top: 0,
                right: 240,
                bottom: rc.bottom - 92,
            };
            let _ = fill_rect(wparam as HDC, &side_rc, CreateSolidBrush(SIDE_COLORREF)).unwrap();
            let play_rc = RECT {
                left: 0,
                top: rc.bottom - 91,
                right: rc.right,
                bottom: rc.bottom,
            };
            let _ = fill_rect(wparam as HDC, &play_rc, CreateSolidBrush(PLAY_COLORREF)).unwrap();
            let playtop_rc = RECT {
                left: 0,
                top: rc.bottom - 92,
                right: rc.right,
                bottom: rc.bottom - 91,
            };
            let _ = fill_rect(
                wparam as HDC,
                &playtop_rc,
                CreateSolidBrush(PLAYTOP_COLORREF),
            )
            .unwrap();
            return 1;
        }
        0x004E => {
            let nmhdr = lparam as *const NMHDR;
            match (*nmhdr).code {
                NM_CUSTOMDRAW => {
                    let draw_st = lparam as *const NMCUSTOMDRAW;
                    match (*draw_st).dwDrawStage {
                        CDDS_PREPAINT => return CDRF_NOTIFYITEMDRAW | CDRF_NOTIFYPOSTPAINT,
                        CDDS_ITEMPREPAINT => match (*draw_st).dwItemSpec {
                            1 => return {
                                return CDRF_SKIPDEFAULT
                            },
                            2 => {
                                let rc = &(*draw_st).rc;
                                Ellipse((*draw_st).hdc, rc.left, rc.top, rc.right, rc.bottom);
                                return CDRF_SKIPDEFAULT
                            }
                            3 => {
                                fill_rect((*draw_st).hdc, &(*draw_st).rc, CreateSolidBrush(PLAYTOP_COLORREF));
                                return CDRF_SKIPDEFAULT
                            },
                            _ => println!("trertert"),
                        },
                        CDDS_POSTPAINT => match (*draw_st).dwItemSpec {
                            3 => {
                                return 0x00000100 as LRESULT
                            },
                            _ => (),
                        }
                        _ => println!("{}", (*draw_st).dwDrawStage as DWORD),
                    }
                },
                _ => (),
            }
        },
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    0
}

pub unsafe extern "system" fn enum_procedure(child: HWND, lparam: LPARAM) -> BOOL {
    match get_window_id(child) {
        Ok(id) => {
            if let Some((x, y)) = match id {
                BTN_ID => match get_window_userdata::<CustomBtn>(child) {
                    Ok(data) => Some(((*data).x, (*data).y)),
                    Err(e) => {
                        println!("Error while getting userdata: {}", e);
                        None
                    },
                },
                _ => None,
            } {
                let (width, height) = *(lparam as *const (u16, u16));
                let nx = (width as f32 * x.0) as i32 + x.1;
                let ny = (height as f32 * y.0) as i32 + y.1;

                if let Err(e) = set_window_pos(child, nx, ny) {
                    println!("Error while setting window position: {}", e);
                }
            }
        }
        Err(e) => println!("Error while getting window id: {}", e),
    }
    return 1;
}
