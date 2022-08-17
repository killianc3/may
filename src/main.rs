use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW, Win32::UI::Shell::SetWindowSubclass,
    Win32::UI::WindowsAndMessaging::*,
};

use may::{buttonproc, create_control, create_window, hiword, loword, ButtonData};

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleW(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = w!("window");
        let button_class = w!("button");

        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance,
            lpszClassName: PCWSTR::from(window_class),
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassW(&wc);
        debug_assert!(atom != 0);

        let hwnd = create_window(window_class, "spotify", [600, 200], instance);

        let mut hwnds: Vec<(HWND, [(f32, i32); 2])> = Vec::new();

        let child = create_control(button_class, [32, 32], hwnd, instance);
        let mut data = ButtonData::new(vec![["btn.ico", "btn_h.ico"], ["btn2.ico", "btn2_h.ico"]], instance)?;
        SetWindowSubclass(child, Some(buttonproc), 0, (&mut data) as *mut _ as usize).ok()?;
        hwnds.push((child, [(0.5, 0), (1.0, -74)]));

        SetPropW(hwnd, w!("hwnds"), HANDLE((&hwnds) as *const _ as isize)).ok()?;

        let mut msg = MSG::default();

        while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
            DispatchMessageA(&msg);
        }

        Ok(())
    }
}

extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg as u32 {
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let _hdc = BeginPaint(hwnd, &mut ps);

                EndPaint(hwnd, &mut ps).ok().unwrap();
                LRESULT(0)
            }
            WM_SIZE => {
                let width = loword(lparam.0 as u32);
                let height = hiword(lparam.0 as u32);
                let handle = GetPropW(hwnd, w!("hwnds"));
                if !handle.is_invalid() {
                    let data = handle.0 as *const Vec<(HWND, [(f32, i32); 2])>;
                    let mut hdwp = BeginDeferWindowPos((*data).len() as i32);
                    for (hwnd, pos) in &*data {
                        let x = (width as f32 * pos[0].0) as i32 + pos[0].1;
                        let y = (height as f32 * pos[1].0) as i32 + pos[1].1;
                        hdwp = DeferWindowPos(hdwp, *hwnd, HWND(0), x, y, 0, 0, SWP_NOSIZE);
                    }
                    EndDeferWindowPos(hdwp).ok().unwrap();
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                RemovePropW(hwnd, w!("hwnds")).ok().unwrap();
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
