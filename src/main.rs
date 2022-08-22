use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW, Win32::System::WindowsProgramming::*,
    Win32::UI::Controls::*, Win32::UI::Shell::SetWindowSubclass, Win32::UI::WindowsAndMessaging::*,
};

use may::{
    buttonproc, create_control, create_window, hiword, loword, trackbarproc, ButtonData, Control,
    TrackbarData,
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
            icons: vec![[
                HICON(LoadImageW(ins, w!("icons/play.ico"), IMAGE_ICON, 0, 0, LR_LOADFROMFILE)?.0),
                HICON(
                    LoadImageW(
                        ins,
                        w!("icons/playh.ico"),
                        IMAGE_ICON,
                        0,
                        0,
                        LR_LOADFROMFILE,
                    )?
                    .0,
                ),
            ]],
            ..Default::default()
        };
        controls.push(Control {
            class: HSTRING::from("Button"),
            proc: Some(buttonproc),
            x: (0.5, 0),
            y: (1.0, -74),
            size: [34, 34],
            data: &mut btn1 as *mut _ as usize,
            ..Default::default()
        });

        let mut tbr1 = TrackbarData::default();
        controls.push(Control {
            class: HSTRING::from("msctls_trackbar32"),
            style: Some(TBS_NOTICKS),
            proc: Some(trackbarproc),
            x: (0.5, -100),
            y: (1.0, -24),
            size: [200, 18],
            data: &mut tbr1 as *mut _ as usize,
            ..Default::default()
        });

        let ptr: *const core::ffi::c_void = &mut controls as *mut _ as *const core::ffi::c_void;
        let _hwnd = create_window(window_class, "spotify", [600, 200], ins, ptr);

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
                    let child =
                        create_control(&control.class, control.style, control.size, hwnd, ins);
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

                SetDCBrushColor(hdc, 0x00000000);
                FillRect(
                    hdc,
                    &RECT {
                        left: 0,
                        top: 0,
                        right: 240,
                        bottom: rc.bottom - 92,
                    },
                    HBRUSH(GetStockObject(DC_BRUSH).0),
                );

                SetDCBrushColor(hdc, 0x00525252);
                FillRect(
                    hdc,
                    &RECT {
                        left: 0,
                        top: rc.bottom - 92,
                        right: rc.right,
                        bottom: rc.bottom - 91,
                    },
                    HBRUSH(GetStockObject(DC_BRUSH).0),
                );
                SetDCBrushColor(hdc, 0x00181818);
                FillRect(
                    hdc,
                    &RECT {
                        left: 0,
                        top: rc.bottom - 91,
                        right: rc.right,
                        bottom: rc.bottom,
                    },
                    HBRUSH(GetStockObject(DC_BRUSH).0),
                );

                LRESULT(0)
            }
            WM_SIZE => {
                let width = loword(lparam.0 as u32) as f32;
                let height = hiword(lparam.0 as u32) as f32;

                let controls = GetPropW(hwnd, w!("controls")).0 as *mut Vec<Control>;
                let mut hdwp = BeginDeferWindowPos((*controls).len() as i32);
                for control in &*controls {
                    let x = (width * control.x.0) as i32 + control.x.1;
                    let y = (height * control.y.0) as i32 + control.y.1;
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
