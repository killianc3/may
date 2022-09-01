use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW, Win32::System::WindowsProgramming::*,
    Win32::UI::Controls::*, Win32::UI::Shell::SetWindowSubclass, Win32::UI::WindowsAndMessaging::*,
};

use may::{
    buttonproc, create_control, create_window, hiword, icon, log_to_phy, log_to_phy_rc, loword,
    phy_to_log, phy_to_log_rc, trackbarproc, ButtonData, Control, Fonts, TrackbarData,
};

fn main() -> Result<()> {
    unsafe {
        let ins = GetModuleHandleW(None)?; // get the module handle required for the suite
        debug_assert!(ins.0 != 0); // panic if the handle is null

        // setup the windows class
        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: ins,
            lpszClassName: PCWSTR::from(&HSTRING::from("window")),
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hIcon: icon("logo.ico", ins)?,
            ..Default::default()
        };

        let atom = RegisterClassW(&wc); // register previous class
        debug_assert!(atom != 0); // panic if registration failed

        // Vector that contains all subclass data
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
            y: (1.0, -76),
            size: (34, 34),
            data: &mut btn1 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn2 = ButtonData {
            icons: vec![[icon("back.ico", ins)?, icon("backh.ico", ins)?]],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("Button"),
            proc: Some(buttonproc),
            x: (0.5, -55),
            y: (1.0, -66),
            size: (14, 14),
            data: &mut btn2 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn3 = ButtonData {
            icons: vec![[icon("next.ico", ins)?, icon("nexth.ico", ins)?]],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("button"),
            proc: Some(buttonproc),
            x: (0.5, 41),
            y: (1.0, -66),
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
            y: (1.0, -68),
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
            y: (1.0, -68),
            size: (26, 26),
            data: &mut btn5 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn6 = ButtonData {
            icons: vec![
                [icon("list.ico", ins)?, icon("listh.ico", ins)?],
                [icon("list2.ico", ins)?, icon("list2h.ico", ins)?],
            ],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("button"),
            proc: Some(buttonproc),
            x: (1.0, -233),
            y: (1.0, -54),
            size: (26, 26),
            data: &mut btn6 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn7 = ButtonData {
            icons: vec![[icon("dev.ico", ins)?, icon("devh.ico", ins)?]],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("button"),
            proc: Some(buttonproc),
            x: (1.0, -197),
            y: (1.0, -54),
            size: (18, 18),
            data: &mut btn7 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn8 = ButtonData {
            icons: vec![
                [icon("soun2.ico", ins)?, icon("soun2h.ico", ins)?],
                [icon("soun.ico", ins)?, icon("sounh.ico", ins)?],
            ],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("button"),
            proc: Some(buttonproc),
            x: (1.0, -165),
            y: (1.0, -54),
            size: (18, 18),
            data: &mut btn8 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn9 = ButtonData {
            icons: vec![[icon("full.ico", ins)?, icon("fullh.ico", ins)?]],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("button"),
            proc: Some(buttonproc),
            x: (1.0, -39),
            y: (1.0, -56),
            size: (16, 16),
            data: &mut btn9 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn10 = ButtonData {
            icons: vec![
                [icon("mic.ico", ins)?, icon("mich.ico", ins)?],
                [icon("mic2.ico", ins)?, icon("mic2h.ico", ins)?],
            ],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("button"),
            proc: Some(buttonproc),
            x: (1.0, -265),
            y: (1.0, -55),
            size: (26, 26),
            data: &mut btn10 as *mut _ as usize,
            ..Default::default()
        });

        let mut btn11 = ButtonData {
            icons: vec![
                [icon("hea2.ico", ins)?, icon("hea2h.ico", ins)?],
                [icon("hea.ico", ins)?, icon("heah.ico", ins)?],
            ],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("button"),
            proc: Some(buttonproc),
            x: (0.0, 184),
            y: (1.0, -54),
            size: (18, 18),
            data: &mut btn11 as *mut _ as usize,
            ..Default::default()
        });

        let mut tbr1 = TrackbarData::default();
        controls.push(Control {
            class: HSTRING::from("msctls_trackbar32"),
            style: Some(TBS_NOTICKS),
            proc: Some(trackbarproc),
            x: (0.5, -210),
            y: (1.0, -32),
            size: (417, 12),
            data: &mut tbr1 as *mut _ as usize,
            ..Default::default()
        });

        let mut tbr2 = TrackbarData::default();
        controls.push(Control {
            class: HSTRING::from("msctls_trackbar32"),
            style: Some(TBS_NOTICKS),
            proc: Some(trackbarproc),
            x: (1.0, -148),
            y: (1.0, -51),
            size: (106, 12),
            data: &mut tbr2 as *mut _ as usize,
            ..Default::default()
        });

        // create pointer to controls
        let ptr: *const core::ffi::c_void = &mut controls as *mut _ as *const core::ffi::c_void;

        // create the main window
        let hwnd = create_window(
            &HSTRING::from("window"),
            "spotify",
            log_to_phy(1080, 480),
            ins,
            ptr,
        );

        // get the handle to the device context
        let hdc = GetDC(hwnd);

        // install font from file
        let result = AddFontResourceExW(
            w!("fonts/gothambold-Bold.otf"),
            FR_PRIVATE,
            core::ptr::null_mut(),
        );
        debug_assert!(result != 0); // panic if installation failed
                                   
        let result = AddFontResourceExW(
            w!("fonts/gothammedium-Medium.otf"),
            FR_PRIVATE,
            core::ptr::null_mut(),
        );
        debug_assert!(result != 0);

        let result = AddFontResourceExW(
            w!("fonts/gothambook-Book.otf"),
            FR_PRIVATE,
            core::ptr::null_mut(),
        );
        debug_assert!(result != 0);

        let result = AddFontResourceExW(
            w!("fonts/gothamlight-Light.otf"),
            FR_PRIVATE,
            core::ptr::null_mut(),
        );
        debug_assert!(result != 0);

        // setup logfont class
        let mut lf1 = LOGFONTW::default();
        lf1.lfHeight = -MulDiv(10, GetDeviceCaps(hdc, LOGPIXELSY), 72);
        for (a, b) in "gothambold".encode_utf16().enumerate() {
            lf1.lfFaceName[a] = b;
        }
        let hfont1 = CreateFontIndirectW(&lf1 as *const _); // get handle to the custom font

        let mut lf2 = LOGFONTW::default();
        lf2.lfHeight = -MulDiv(10, GetDeviceCaps(hdc, LOGPIXELSY), 72);
        for (a, b) in "gothammedium".encode_utf16().enumerate() {
            lf2.lfFaceName[a] = b;
        }
        let hfont2 = CreateFontIndirectW(&lf2 as *const _);

        let mut lf3 = LOGFONTW::default();
        lf3.lfHeight = -MulDiv(8, GetDeviceCaps(hdc, LOGPIXELSY), 72);
        for (a, b) in "gothamlight".encode_utf16().enumerate() {
            lf3.lfFaceName[a] = b;
        }
        let hfont3 = CreateFontIndirectW(&lf3 as *const _);

        let mut lf4 = LOGFONTW::default();
        lf4.lfHeight = -MulDiv(8, GetDeviceCaps(hdc, LOGPIXELSY), 72);
        for (a, b) in "gothambook".encode_utf16().enumerate() {
            lf4.lfFaceName[a] = b;
        }
        let hfont4 = CreateFontIndirectW(&lf4 as *const _);

        let fonts = Fonts {
            book: hfont4,
            light: hfont3,
            medium: hfont2,
            bold: hfont1,
        };

        // pass custom fonts to main window procedure
        SetPropW(hwnd, w!("fonts"), HANDLE(&fonts as *const _ as isize)); 

        let garo = icon("garo.ico", ins)?; // get handle to an icon

        // pass custom fonts to main window procedure
        SetPropW(hwnd, w!("garo"), HANDLE(&garo as *const _ as isize));

        // the app runtime
        let mut msg = MSG::default();
        while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
            DispatchMessageA(&msg);
        }

        // remove custom fonts installation
        RemoveFontResourceExW(
            w!("fonts/gothambold-Bold.otf"),
            FR_PRIVATE.0,
            core::ptr::null_mut(),
        )
        .ok()?;
        RemoveFontResourceExW(
            w!("fonts/gothammedium-Medium.otf"),
            FR_PRIVATE.0,
            core::ptr::null_mut(),
        )
        .ok()?;
        RemoveFontResourceExW(
            w!("fonts/gothambook-Book.otf"),
            FR_PRIVATE.0,
            core::ptr::null_mut(),
        )
        .ok()?;
        RemoveFontResourceExW(
            w!("fonts/gothamlight-Light.otf"),
            FR_PRIVATE.0,
            core::ptr::null_mut(),
        )
        .ok()?;

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
                let hdc = BeginPaint(hwnd, &mut ps);

                let mut rc = RECT::default();
                GetClientRect(hwnd, &mut rc);
                rc = phy_to_log_rc(rc);

                let fonts = GetPropW(hwnd, w!("fonts")).0 as *const Fonts;
                let garo = GetPropW(hwnd, w!("garo")).0 as *const HICON;

                SetBkMode(hdc, TRANSPARENT);
                SetTextColor(hdc, 0x00A7A7A7);

                let old_f = SelectObject(hdc, (*fonts).book);
                let (x, y) = log_to_phy((rc.right as f32 * 0.5) as i32 - 230, rc.bottom - 32);
                TextOutW(hdc, x, y, w!("0:00").as_wide());
                let (x, y) = log_to_phy((rc.right as f32 * 0.5) as i32 + 210, rc.bottom - 32);
                TextOutW(hdc, x, y, w!("-1:43").as_wide());
                let (x, y) = log_to_phy(85, rc.bottom - 41);
                TextOutW(hdc, x, y, w!("LVZ, Max DLG").as_wide());
                let (x, y) = log_to_phy(85, rc.bottom - 62);
                SetTextColor(hdc, 0x00FFFFFF);
                SelectObject(hdc, (*fonts).medium);
                TextOutW(hdc, x, y, w!("Garo").as_wide());
                SelectObject(hdc, old_f);

                let (width, height) = log_to_phy(56, 56);
                let (x, y) = log_to_phy(15, rc.bottom - 73);
                DrawIconEx(hdc, x, y, *garo, width, height, 0, HBRUSH(0), DI_NORMAL);

                EndPaint(hwnd, &mut ps).ok().unwrap();
                LRESULT(0)
            }
            WM_ERASEBKGND => {
                let hdc = HDC(wparam.0 as isize);
                let mut rc = RECT::default();
                GetClientRect(hwnd, &mut rc);
                rc = phy_to_log_rc(rc);

                SetDCBrushColor(hdc, 0x00121212);
                let rc_temp = RECT {
                    left: 240,
                    top: 0,
                    right: rc.right,
                    bottom: rc.bottom - 92,
                };
                FillRect(
                    hdc,
                    &log_to_phy_rc(rc_temp),
                    HBRUSH(GetStockObject(DC_BRUSH).0),
                );

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
                RemovePropW(hwnd, w!("fonts")).ok().unwrap();
                RemovePropW(hwnd, w!("garo")).ok().unwrap();
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
