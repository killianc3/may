#![cfg(windows)]

use core::{
    ffi::c_void,
    ptr::{null, null_mut},
};

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
pub type Colorref = Dword;
pub type Dword = u32;
pub type DwordPtr = UlongPtr;
pub type Handle = Pvoid;
pub type Hbrush = Handle;
pub type Hcursor = Handle;
pub type Hdc = Handle;
pub type Hdwp = Handle;
pub type Hgdiobj = *mut c_void;
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
pub type Subclassproc = Option<
    unsafe extern "system" fn(
        hwnd: Hwnd,
        uMsg: Uint,
        wParam: Wparam,
        lParam: Lparam,
        uIdSubclass: UintPtr,
        dwRefData: DwordPtr,
    ) -> Lresult,
>;

pub const CS_HREDRAW: u32 = 0x0002;
pub const CS_VREDRAW: u32 = 0x0001;

pub const CW_USEDEFAULT: i32 = 0x80000000_u32 as i32;

pub const LR_LOADFROMFILE: u32 = 0x00000010;

pub const SW_SHOW: i32 = 5;

pub const WM_QUIT: u32 = 0x0012;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_SIZE: u32 = 0x0005;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_ERASEBKGND: u32 = 0x0014;
pub const WM_NCDESTROY: u32 = 0x0082;
pub const WM_MOUSEMOVE: u32 = 0x0200;
pub const WM_LBUTTONDOWN: u32 = 0x0201;
pub const WM_LBUTTONUP: u32 = 0x0202;
pub const WM_MOUSELEAVE: u32 = 0x02a3;

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
pub const WS_CHILD: u32 = 0x40000000;
pub const WS_VISIBLE: u32 = 0x10000000;

pub const fn makeintresourcew(i: Word) -> Lpwstr {
    i as UlongPtr as Lpwstr
}

pub const fn rgb(r: u32, g: u32, b: u32) -> Colorref {
    r | (g << 8) | (b << 16)
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

#[repr(C)]
pub struct NMHDR {
    pub hwnd_from: Hwnd,
    pub id_from: UintPtr,
    pub code: Uint,
}
unsafe_impl_default_zeroed!(NMHDR);

#[repr(C)]
pub struct NMBCHOTITEM {
    pub hdr: NMHDR,
    pub dw_flags: Dword,
}

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
    pub fn BeginDeferWindowPos(nNumWindiw: Int) -> Hdwp;
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
    pub fn DeferWindowPos(
        hWinPosInfo: Hdwp,
        hWnd: Hwnd,
        hWndInsertAfter: Hwnd,
        x: Int,
        y: Int,
        cx: Int,
        cy: Int,
        uFlags: Uint,
    ) -> Hdwp;
    pub fn DefWindowProcW(hWnd: Hwnd, Msg: Uint, wParam: Wparam, lParam: Lparam) -> Lresult;
    pub fn DestroyWindow(hWnd: Hwnd) -> Bool;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> Lresult;
    pub fn DrawIcon(hDC: Hdc, X: Int, Y: Int, hIcon: Hicon) -> Bool;
    pub fn EndDeferWindowPos(hWinPosInfo: Hdwp) -> Bool;
    pub fn EndPaint(hWnd: Hwnd, lpPaint: *const PAINTSTRUCT) -> Bool;
    pub fn FillRect(hDC: Hdc, lprc: *const RECT, hbr: Hbrush) -> Int;
    pub fn GetPropW(hWnd: Hwnd, lpString: Lpcwstr) -> Handle;
    pub fn GetMessageW(
        lpMsg: *const MSG,
        hWnd: Hwnd,
        wMsgFilterMin: Uint,
        wMsgFilterMax: Uint,
    ) -> Bool;
    pub fn InvalidateRect(hWnd: Hwnd, lprc: *const RECT, bErase: Bool) -> Bool;
    pub fn LoadCursorW(hInstance: Hinstance, lpCursorName: Lpcwstr) -> Hcursor;
    pub fn LoadImageW(
        hInst: Hinstance,
        name: Lpcwstr,
        r#type: Uint,
        cx: Int,
        cy: Int,
        fuLoad: Uint,
    ) -> Handle;
    pub fn PostQuitMessage(nExitCode: Int);
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> Atom;
    pub fn RemovePropW(hWnd: Hwnd, lpString: Lpcwstr) -> Handle;
    pub fn SetPropW(hWnd: Hwnd, lpString: Lpcwstr, hData: Handle) -> Bool;
    pub fn ShowWindow(hWnd: Hwnd, nCmdShow: Int) -> Bool;
    pub fn TranslateMessage(lpMsg: *const MSG) -> Bool;
    pub fn UpdateWindow(hWnd: Hwnd) -> Bool;
}

#[link(name = "Comctl32")]
extern "system" {
    pub fn DefSubclassProc(hWnd: Hwnd, uMsg: Uint, wParam: Wparam, lParam: Lparam) -> Lresult;
    pub fn SetWindowSubclass(
        hWnd: Hwnd,
        pfnSubclass: Subclassproc,
        uIdSubclass: UintPtr,
        dwRefData: DwordPtr,
    ) -> Bool;
}

#[link(name = "Gdi32")]
extern "system" {
    pub fn CreateSolidBrush(color: Colorref) -> Hbrush;
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

pub fn create_app_window(
    class_name: &str,
    window_name: &str,
    width: i32,
    height: i32,
    instance: Hinstance,
) -> Result<Hwnd, Win32Error> {
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            wide_null(class_name).as_ptr(),
            wide_null(window_name).as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_CLIPSIBLINGS,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            null_mut(),
            null_mut(),
            instance,
            null_mut(),
        )
    };
    if hwnd.is_null() {
        Err(get_last_error())
    } else {
        Ok(hwnd)
    }
}

pub fn create_control_window(
    class_name: &str,
    width: i32,
    height: i32,
    parent: Hwnd,
    id: u32,
    instance: Hinstance,
) -> Result<Hwnd, Win32Error> {
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            wide_null(class_name).as_ptr(),
            null_mut(),
            WS_CHILD | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            parent,
            id as Hmenu,
            instance,
            null_mut(),
        )
    };
    if hwnd.is_null() {
        Err(get_last_error())
    } else {
        Ok(hwnd)
    }
}

pub fn set_window_subclass(
    hwnd: Hwnd,
    pfnsubclass: Subclassproc,
    uidsubclass: UintPtr,
) -> Result<(), ()> {
    if unsafe { SetWindowSubclass(hwnd, pfnsubclass, uidsubclass, 0) } == 0 {
        Err(())
    } else {
        Ok(())
    }
}

pub fn set_prop<T>(hwnd: Hwnd, string: &str, data: &mut T) -> Result<(), Win32Error> {
    if unsafe { SetPropW(hwnd, wide_null(string).as_ptr(), data as *mut _ as Handle) } == 0 {
        Err(get_last_error())
    } else {
        Ok(())
    }
}

pub fn get_prop<T>(hwnd: Hwnd, string: &str) -> Result<&mut T, ()> {
    let handle = unsafe { GetPropW(hwnd, wide_null(string).as_ptr()) };
    if handle.is_null() {
        Err(())
    } else {
        Ok(unsafe { &mut *(handle as *mut T) })
    }
}

pub fn load_icon(instance: Hinstance, filename: &str) -> Result<Hicon, Win32Error> {
    let handle = unsafe {
        LoadImageW(
            instance,
            wide_null(filename).as_ptr(),
            1,
            0,
            0,
            LR_LOADFROMFILE,
        )
    };
    if handle.is_null() {
        Err(get_last_error())
    } else {
        Ok(handle)
    }
}

pub fn create_brush(r: u32, g: u32, b: u32) -> Result<Hbrush, ()> {
    let hbrush = unsafe { CreateSolidBrush(rgb(r, g, b)) };
    if hbrush.is_null() {
        Err(())
    } else {
        Ok(hbrush)
    }
}

pub fn fill_rect(hdc: Hdc, rc: &RECT, hbr: Hbrush) -> Result<(), ()> {
    if unsafe { FillRect(hdc, rc, hbr) } != 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn draw_icon(hdc: Hdc, hicon: Hicon) -> Result<(), Win32Error> {
    if unsafe { DrawIcon(hdc, 0, 0, hicon) } != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

pub fn begin_defer_window_pos(nnumwindows: Int) -> Result<Hdwp, Win32Error> {
    let hdwp = unsafe { BeginDeferWindowPos(nnumwindows) };
    if hdwp.is_null() {
        Err(get_last_error())
    } else {
        Ok(hdwp)
    }
}

pub fn end_defer_window_pos(hwinposinfo: Hdwp) -> Result<(), Win32Error> {
    if unsafe { EndDeferWindowPos(hwinposinfo) } != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

pub fn hiword(l: Dword) -> Word {
    ((l >> 16) & 0xffff) as Word
}

pub fn loword(l: Dword) -> Word {
    (l & 0xffff) as Word
}
