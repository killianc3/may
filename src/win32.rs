#![cfg(windows)]

use core::{ptr::{null, null_mut}, ffi::c_void};

macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }
    };
}

struct OnDropLocalFree(Hlocal);
impl Drop for OnDropLocalFree {
    fn drop(&mut self) {
        unsafe { LocalFree(self.0) };
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Win32Error(pub Dword);
impl std::error::Error for Win32Error {}
impl core::fmt::Display for Win32Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0 & (1 << 29) > 0 {
            return write!(f, "Win32ApplicationError({})", self.0);
        }
        pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: Dword = 0x00000100;
        pub const FORMAT_MESSAGE_FROM_SYSTEM: Dword = 0x00001000;
        pub const FORMAT_MESSAGE_IGNORE_INSERTS: Dword = 0x00000200;
        let dw_flags = FORMAT_MESSAGE_ALLOCATE_BUFFER
            | FORMAT_MESSAGE_FROM_SYSTEM
            | FORMAT_MESSAGE_IGNORE_INSERTS;
        let lp_source = null_mut();
        let dw_message_id = self.0;
        let dw_language_id = 0;
        let mut buffer: *mut u16 = null_mut();
        let lp_buffer = &mut buffer as *mut *mut u16 as *mut u16;
        let n_size = 0;
        let arguments = null_mut();
        let tchar_count_excluding_null = unsafe {
            FormatMessageW(
                dw_flags,
                lp_source,
                dw_message_id,
                dw_language_id,
                lp_buffer,
                n_size,
                arguments,
            )
        };
        if tchar_count_excluding_null == 0 || buffer.is_null() {
            return Err(core::fmt::Error);
        } else {
            let _on_drop = OnDropLocalFree(buffer as Hlocal);
            let buffer_slice: &[u16] =
                unsafe { core::slice::from_raw_parts(buffer, tchar_count_excluding_null as usize) };
            for decode_result in core::char::decode_utf16(buffer_slice.iter().copied()) {
                match decode_result {
                    Ok('\r') | Ok('\n') => write!(f, " ")?,
                    Ok(ch) => write!(f, "{}", ch)?,
                    Err(_) => write!(f, "�")?,
                }
            }
            Ok(())
        }
    }
}

pub type Atom = Word;
pub type Bool = i32;
pub type Byte = u8;
pub type Dword = u32;
pub type Handle = Pvoid;
pub type Hbrush = Handle;
pub type Hcursor = Handle;
pub type Hdc = Handle;
pub type Hicon = Handle;
pub type Hinstance = Handle;
pub type Hlocal = Handle;
pub type Hmenu = Handle;
pub type Hmodule = Handle;
pub type Hwnd = Handle;
pub type Int = i32;
pub type Long = i32;
pub type LongPtr = isize;
pub type Lparam = LongPtr;
pub type Lpcvoid = *const c_void;
pub type Lpcwstr = *const Wchar;
pub type Lpvoid = *mut c_void;
pub type Lpwstr = *mut Wchar;
pub type Lpcstr = *mut Wchar;
pub type Lresult = LongPtr;
pub type Pvoid = *mut c_void;
pub type Uint = u32;
pub type UintPtr = usize;
pub type UlongPtr = usize;
pub type VaList = *mut i8;
pub type Wchar = u16;
pub type Word = u16;
pub type Wparam = UintPtr;

pub type Wndproc = Option<
    unsafe extern "system" fn(hwnd: Hwnd, uMsg: Uint, wParam: Wparam, lParam: Lparam) -> Lresult,
>;

pub const CS_HREDRAW: u32 = 0x0002;
pub const CS_VREDRAW: u32 = 0x0001;

pub const CW_USEDEFAULT: i32 = 0x80000000_u32 as i32;

pub const SW_SHOW: i32 = 5;

pub const WM_QUIT: u32 = 0x0012;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_SIZE: u32 = 0x0005;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_ERASEBKGND: u32 = 0x0014;

pub const WS_CLIPCHILDREN: u32 = 0x02000000;
pub const WS_CLIPSIBLINGS: u32 = 0x04000000;
pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub const fn makeintresourcew(i: Word) -> Lpwstr {
    i as UlongPtr as Lpwstr
}

pub enum IDCursor {
    AppStarting = 32650,
    Arrow = 32512,
    Cross = 32515,
    Help = 32651,
    IBeam = 32513,
    No = 32648,
    SizeAll = 32646,
    SizeNeSw = 32643,
    SizeNS = 32645,
    SizeNwSe = 32642,
    SizeWE = 32644,
    UpArrow = 32516,
    Wait = 32514,
}

#[repr(C)]
pub struct WNDCLASSW {
    pub style: Uint,
    pub lpfn_wnd_proc: Wndproc,
    pub cb_cls_extra: Int,
    pub cb_wnd_extra: Int,
    pub h_instance: Hinstance,
    pub h_icon: Hicon,
    pub h_cursor: Hcursor,
    pub hbr_background: Hbrush,
    pub lpsz_menu_name: Lpcwstr,
    pub lpsz_class_name: Lpcwstr,
}
unsafe_impl_default_zeroed!(WNDCLASSW);

#[repr(C)]
pub struct MSG {
    pub hwnd: Hwnd,
    pub message: Uint,
    pub w_param: Wparam,
    pub l_param: Lparam,
    pub time: Dword,
    pub pt: POINT,
    pub l_private: Dword,
}
unsafe_impl_default_zeroed!(MSG);

#[repr(C)]
pub struct POINT {
    pub x: Long,
    pub y: Long,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: Hdc,
    pub f_erase: Bool,
    pub rc_paint: RECT,
    pub f_restore: Bool,
    pub f_inc_update: Bool,
    pub rgb_reserved: [Byte; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

#[repr(C)]
pub struct RECT {
    pub left: Long,
    pub top: Long,
    pub right: Long,
    pub bottom: Long,
}
unsafe_impl_default_zeroed!(RECT);

#[link(name = "Kernel32")]
extern "system" {
    pub fn FormatMessageW(
        dwFlags: Dword,
        lpSource: Lpcvoid,
        dwMessageId: Dword,
        dwLanguageId: Dword,
        lpBuffer: Lpwstr,
        nSize: Dword,
        Arguments: VaList,
    ) -> Dword;
    pub fn GetLastError() -> Dword;
    pub fn GetModuleHandleW(lpModuleName: Lpcwstr) -> Hmodule;
    pub fn LocalFree(hMem: Hlocal) -> Hlocal;
}

#[link(name = "User32")]
extern "system" {
    pub fn BeginPaint(hWnd: Hwnd, lpPaint: *const PAINTSTRUCT) -> Hdc;
    pub fn CreateWindowExW(
        dwExStyle: Dword,
        lpClassName: Lpcwstr,
        lpWindowName: Lpcwstr,
        dwStyle: Dword,
        X: Int,
        Y: Int,
        nWidth: Int,
        nHeight: Int,
        hWndParent: Hwnd,
        hMenu: Hmenu,
        hInstance: Hinstance,
        lpParam: Lpvoid,
    ) -> Hwnd;
    pub fn DefWindowProcW(hWnd: Hwnd, Msg: Uint, wParam: Wparam, lParam: Lparam) -> Lresult;
    pub fn DestroyWindow(hWnd: Hwnd) -> Bool;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> Lresult;
    pub fn EndPaint(hWnd: Hwnd, lpPaint: *const PAINTSTRUCT) -> Bool;
    pub fn GetMessageW(
        lpMsg: *const MSG,
        hWnd: Hwnd,
        wMsgFilterMin: Uint,
        wMsgFilterMax: Uint,
    ) -> Bool;
    pub fn LoadCursorW(hInstance: Hinstance, lpCursorName: Lpcwstr) -> Hcursor;
    pub fn PostQuitMessage(nExitCode: Int);
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> Atom;
    pub fn ShowWindow(hWnd: Hwnd, nCmdShow: Int) -> Bool;
    pub fn TranslateMessage(lpMsg: *const MSG) -> Bool;
}

pub fn get_process_handle() -> Hmodule {
    unsafe { GetModuleHandleW(null()) }
}

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

pub fn load_predefined_cursor(cursor: IDCursor) -> Result<Hcursor, Win32Error> {
    let hcursor = unsafe { LoadCursorW(null_mut(), makeintresourcew(cursor as Word)) };
    if hcursor.is_null() {
        Err(get_last_error())
    } else {
        Ok(hcursor)
    }
}

pub fn get_last_error() -> Win32Error {
    Win32Error(unsafe { GetLastError() })
}

pub fn register_class(window_class: &WNDCLASSW) -> Result<Atom, Win32Error> {
    let atom = unsafe { RegisterClassW(window_class) };
    if atom == 0 {
        Err(get_last_error())
    } else {
        Ok(atom)
    }
}

pub fn get_any_message() -> Result<MSG, Win32Error> {
    let mut msg = MSG::default();
    let output = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
    if output == -1 {
        Err(get_last_error())
    } else {
        Ok(msg)
    }
}

pub fn translate_message(msg: &MSG) -> bool {
    0 != unsafe { TranslateMessage(msg) }
}

pub fn begin_paint(hwnd: Hwnd) -> Result<(Hdc, PAINTSTRUCT), Win32Error> {
    let mut ps = PAINTSTRUCT::default();
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
    if hdc.is_null() {
        Err(get_last_error())
    } else {
        Ok((hdc, ps))
    }
}

pub fn end_paint(hwnd: Hwnd, ps: &PAINTSTRUCT) {
    unsafe { EndPaint(hwnd, ps) };
}

pub fn post_quit_message(exit_code: Int) {
    unsafe { PostQuitMessage(exit_code) }
}
