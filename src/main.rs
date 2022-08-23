use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW, Win32::UI::Controls::*,
    Win32::UI::Shell::SetWindowSubclass, Win32::UI::WindowsAndMessaging::*,
};

use may::{
    buttonproc, create_control, create_window, hiword, icon, log_to_phy, log_to_phy_rc, loword,
    phy_to_log, phy_to_log_rc, trackbarproc, ButtonData, Control, TrackbarData,
};

fn main() -> Result<()> {
    unsafe {
        let ins = GetModuleHandleW(None)?;
        debug_assert!(ins.0 != 0);

        let window_class = w!("window");

        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: ins,
            lpszClassName: PCWSTR::from(window_class),
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassW(&wc);
        debug_assert!(atom != 0);

        let mut controls: Vec<Control> = Vec::new();

        let mut btn1 = ButtonData {
            icons: vec![
                [icon("play.ico", ins)?, icon("playh.ico", ins)?],
                [icon("play2.ico", ins)?, icon("play2h.ico", ins)?],
            ],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("Button"),
            proc: Some(buttonproc),
            x: (0.5, -17),
            y: (1.0, -79),
            size: (34, 34),
            data: &mut btn1 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn2 = ButtonData {
            icons: vec![
                [icon("back.ico", ins)?, icon("backh.ico", ins)?],
            ],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("Button"),
            proc: Some(buttonproc),
            x: (0.5, -55),
            y: (1.0, -69),
            size: (14, 14),
            data: &mut btn2 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn3 = ButtonData {
            icons: vec![
                [icon("next.ico", ins)?, icon("nexth.ico", ins)?],
            ],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("button"),
            proc: Some(buttonproc),
            x: (0.5, 41),
            y: (1.0, -69),
            size: (14, 14),
            data: &mut btn3 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn4 = ButtonData {
            icons: vec![
                [icon("shuf.ico", ins)?, icon("shufh.ico", ins)?],
                [icon("shuf2.ico", ins)?, icon("shuf2h.ico", ins)?],
            ],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("button"),
            proc: Some(buttonproc),
            x: (0.5, -101),
            y: (1.0, -71),
            size: (26, 26),
            data: &mut btn4 as *mut _ as usize,
            ..Default::default()
        });


        let mut btn5 = ButtonData {
            icons: vec![
                [icon("rep.ico", ins)?, icon("reph.ico", ins)?],
                [icon("rep2.ico", ins)?, icon("rep2h.ico", ins)?],
                [icon("rep3.ico", ins)?, icon("rep3h.ico", ins)?],
            ],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("button"),
            proc: Some(buttonproc),
            x: (0.5, 75),
            y: (1.0, -71),
            size: (26, 26),
            data: &mut btn5 as *mut _ as usize,
            ..Default::default()
        });

        /*
        let mut tbr1 = TrackbarData::default();
        controls.push(Control {
            class: HSTRING::from("msctls_trackbar32"),
            style: Some(TBS_NOTICKS),
            proc: Some(trackbarproc),
            x: (0.5, -319),
            y: (1.0, -35),
            size: (638, 18),
            data: &mut tbr1 as *mut _ as usize,
            ..Default::default()
        });*/

        let ptr: *const core::ffi::c_void = &mut controls as *mut _ as *const core::ffi::c_void;
        let _hwnd = create_window(window_class, "spotify", log_to_phy(600, 200), ins, ptr);

        /*let result = AddFontResourceExW(w!("GothamBook.ttf"), FR_PRIVATE, core::ptr::null_mut());
        debug_assert!(result != 0);

        let mut lg = LOGFONTW::default();
        lg.lfHeight = -MulDiv(8, GetDeviceCaps(hdc, LOGPIXELSY), 72);
        for (a, b) in "Gotham".encode_utf16().enumerate() {
            lg.lfFaceName[a] = b;
        }

        let hfont = CreateFontIndirectW(&lg as *const _);
        SetPropW(hwnd, w!("font"), HANDLE(&hfont as *const _ as isize));*/

        let mut msg = MSG::default();

        while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
            DispatchMessageA(&msg);
        }

        //RemoveFontResourceExW(w!("GothamBook.ttf"), FR_PRIVATE.0, core::ptr::null_mut()).ok()?;

        Ok(())
    }
}

extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg as u32 {
            WM_CREATE => {
                let crt = lparam.0 as *const CREATESTRUCTW;
                let ins = (*crt).hInstance;
                let controls = (*crt).lpCreateParams as *mut Vec<Control>;
                for control in &mut *controls {
                    let child = create_control(
                        &control.class,
                        control.style,
                        log_to_phy(control.size.0, control.size.1),
                        hwnd,
                        ins,
                    );
                    SetWindowSubclass(child, control.proc, 0, control.data);
                    control.hwnd = Some(child);
                }

                SetPropW(hwnd, w!("controls"), HANDLE(controls as *mut _ as isize))
                    .ok()
                    .unwrap();

                LRESULT(0)
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let _hdc = BeginPaint(hwnd, &mut ps);

                EndPaint(hwnd, &mut ps).ok().unwrap();
                LRESULT(0)
            }
            WM_ERASEBKGND => {
                let hdc = HDC(wparam.0 as isize);
                let mut rc = RECT::default();
                GetClientRect(hwnd, &mut rc);
                rc = phy_to_log_rc(rc);

                SetDCBrushColor(hdc, 0x00000000);
                let rc_temp = RECT {
                    left: 0,
                    top: 0,
                    right: 240,
                    bottom: rc.bottom - 92,
                };
                FillRect(
                    hdc,
                    &log_to_phy_rc(rc_temp),
                    HBRUSH(GetStockObject(DC_BRUSH).0),
                );

                SetDCBrushColor(hdc, 0x00525252);
                let rc_temp = RECT {
                    left: 0,
                    top: rc.bottom - 92,
                    right: rc.right,
                    bottom: rc.bottom - 91,
                };
                FillRect(
                    hdc,
                    &log_to_phy_rc(rc_temp),
                    HBRUSH(GetStockObject(DC_BRUSH).0),
                );

                SetDCBrushColor(hdc, 0x00181818);
                let rc_temp = RECT {
                    left: 0,
                    top: rc.bottom - 91,
                    right: rc.right,
                    bottom: rc.bottom,
                };
                FillRect(
                    hdc,
                    &log_to_phy_rc(rc_temp),
                    HBRUSH(GetStockObject(DC_BRUSH).0),
                );

                LRESULT(0)
            }
            WM_SIZE => {
                let (width, height) = phy_to_log(
                    loword(lparam.0 as u32) as i32,
                    hiword(lparam.0 as u32) as i32,
                );

                let controls = GetPropW(hwnd, w!("controls")).0 as *mut Vec<Control>;
                let mut hdwp = BeginDeferWindowPos((*controls).len() as i32);
                for control in &*controls {
                    let x = (width as f32 * control.x.0) as i32 + control.x.1;
                    let y = (height as f32 * control.y.0) as i32 + control.y.1;
                    let (x, y) = log_to_phy(x, y);
                    hdwp = DeferWindowPos(hdwp, control.hwnd, HWND(0), x, y, 0, 0, SWP_NOSIZE);
                }
                EndDeferWindowPos(hdwp).ok().unwrap();
                LRESULT(0)
            }
            WM_DESTROY => {
                RemovePropW(hwnd, w!("controls")).ok().unwrap();
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
