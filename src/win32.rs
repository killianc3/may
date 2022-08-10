#![cfg(windows)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::c_void;
use core::ptr::{null, null_mut};

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

pub type c_char = i8;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_ushort = u16;
pub type c_long = i32;
pub type c_ulong = u32;
pub type va_list = *mut c_char;
pub type wchar_t = u16;
pub type ATOM = WORD;
pub type BYTE = u8;
pub type BOOL = c_int;
pub type DWORD = c_ulong;
pub type DWORD_PTR = ULONG_PTR;
pub type COLORREF = u32;
pub type HANDLE = PVOID;
pub type HBRUSH = HANDLE;
pub type HCURSOR = HICON;
pub type HDC = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HICON = HANDLE;
pub type HLOCAL = HANDLE;
pub type HMENU = HANDLE;
pub type HMODULE = HINSTANCE;
pub type HRGN = HANDLE;
pub type HWND = HANDLE;
pub type LONG = c_long;
pub type LONG_PTR = isize;
pub type LPARAM = LONG_PTR;
pub type LPCVOID = *const c_void;
pub type LPCWSTR = *const WCHAR;
pub type LPWSTR = *mut WCHAR;
pub type LPVOID = *mut c_void;
pub type LRESULT = LONG_PTR;
pub type PVOID = *mut c_void;
pub type UINT = c_uint;
pub type UINT_PTR = usize;
pub type ULONG_PTR = usize;
pub type WCHAR = wchar_t;
pub type WORD = c_ushort;
pub type WPARAM = UINT_PTR;

pub type WNDPROC = Option<
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;
pub type ENUMCHILDPROC = Option<unsafe extern "system" fn(hwnd: HWND, lParam: LPARAM) -> BOOL>;

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

pub enum SysColor {
    _3dDarkShadow = 21,
    _3dLight = 22,
    ActiveBorder = 10,
    ActiveCaption = 2,
    AppWorkspace = 12,
    ButtonFace = 15,
    ButtonHighlight = 20,
    ButtonShadow = 16,
    ButtonText = 18,
    CaptionText = 9,
    Desktop = 1,
    GradientActiveCaption = 27,
    GradientInactiveCaption = 28,
    GrayText = 17,
    Highlight = 13,
    HighlightText = 14,
    HotLight = 26,
    InactiveBorder = 11,
    InactiveCaption = 3,
    InactiveCaptionText = 19,
    InfoBackground = 24,
    InfoText = 23,
    Menu = 4,
    MenuHighlight = 29,
    MenuBar = 30,
    MenuText = 7,
    ScrollBar = 0,
    Window = 5,
    WindowFrame = 6,
    WindowText = 8,
}

#[repr(C)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}
unsafe_impl_default_zeroed!(WNDCLASSW);

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);

#[repr(C)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [BYTE; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

#[repr(C)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
unsafe_impl_default_zeroed!(RECT);

#[repr(C)]
pub struct CREATESTRUCTW {
    pub lpCreateParams: LPVOID,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: HWND,
    pub cy: c_int,
    pub cx: c_int,
    pub y: c_int,
    pub x: c_int,
    pub style: LONG,
    pub lpszName: LPCWSTR,
    pub lpszClass: LPCWSTR,
    pub dwExStyle: DWORD,
}
unsafe_impl_default_zeroed!(CREATESTRUCTW);

#[repr(C)]
pub struct DRAWITEMSTRUCT {
    pub CtlType: UINT,
    pub CtlID: UINT,
    pub itemID: UINT,
    pub itemAction: UINT,
    pub itemState: UINT,
    pub hwndItem: HWND,
    pub hDC: HDC,
    pub rcItem: RECT,
    pub itemData: ULONG_PTR,
}
unsafe_impl_default_zeroed!(DRAWITEMSTRUCT);

#[repr(C)]
pub struct NMHDR {
    pub hwndFrom: HWND,
    pub idFrom: usize, // louche 
    pub code: UINT,
}
unsafe_impl_default_zeroed!(NMHDR);

#[repr(C)]
pub struct NMCUSTOMDRAW {
    pub hdr: NMHDR,
    pub dwDrawStage: DWORD,
    pub hdc: HDC,
    pub rc: RECT,
    pub dwItemSpec: DWORD_PTR,
    pub uItemState: UINT,
    pub lItemlParam: LPARAM,
}
unsafe_impl_default_zeroed!(NMCUSTOMDRAW);

pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;
pub const WS_CLIPCHILDREN: u32 = 0x02000000;
pub const WS_CLIPSIBLINGS: u32 = 0x04000000;
pub const WS_CHILD: u32 = 0x40000000;
pub const WS_VISIBLE: u32 = 0x10000000;

pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_QUIT: u32 = 0x0012;
pub const WM_COMMAND: u32 = 0x0111;
pub const WM_SIZE: u32 = 0x0005;
pub const WM_DRAWITEM: u32 = 0x002B;
pub const WM_ERASEBKGND: u32 = 0x0014;
pub const WM_NOTIFY: u32 = 0x004E;

pub const COLOR_WINDOW: u32 = 5;
pub const MB_OKCANCEL: u32 = 1;
pub const IDOK: c_int = 1;
pub const GWLP_USERDATA: c_int = -21;
pub const IMAGE_ICON: u32 = 1;
pub const SIDE_COLORREF: COLORREF = 0x00000000;
pub const PLAY_COLORREF: COLORREF = 0x00181818;
pub const PLAYTOP_COLORREF: COLORREF = 0x00282828;
pub const ODT_BUTTON: UINT = 4;
pub const NM_CUSTOMDRAW: UINT = 4_294_967_284u32;

pub const DI_NORMAL: u32 = 0x0003;
pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;
pub const SW_SHOW: c_int = 5;
pub const BS_OWNERDRAW: u32 = 0x0000000B;
pub const LR_LOADFROMFILE: u32 = 0x00000010;
pub const fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR {
    i as ULONG_PTR as LPWSTR
}

pub const TBCD_TICS: DWORD_PTR = 0x0001;
pub const TBCD_THUMB: DWORD_PTR = 0x0002;
pub const TBCD_CHANNEL: DWORD_PTR = 0x0003;

pub const CDDS_PREPAINT: DWORD = 0x00000001;
pub const CDDS_ITEMPREPAINT: DWORD = 0x00010001;
pub const CDDS_POSTPAINT: DWORD = 0x00000002;
pub const CDDS_DOERASE: DWORD = 0x00000008;
pub const CDRF_DOERASE: LRESULT = 0x00000008;
pub const CDRF_NOTIFYITEMDRAW: LRESULT = 0x00000020;
pub const CDRF_NOTIFYPOSTPAINT: LRESULT = 0x00000010;
pub const CDRF_SKIPDEFAULT: LRESULT = 0x00000004;

pub const CS_HREDRAW: u32 = 0x0002;
pub const CS_VREDRAW: u32 = 0x0001;

pub const BTN_ID: isize = 1;

#[link(name = "Gdi32")]
extern "system" {
    pub fn CreateRoundRectRgn(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        w: c_int,
        h: c_int,
    ) -> HRGN;
    pub fn CreateSolidBrush(color: COLORREF) -> HBRUSH;
    pub fn Ellipse(hdc: HDC, left: c_int, top: c_int, right: c_int, bottom: c_int) -> BOOL;
}

#[link(name = "Kernel32")]
extern "system" {
    pub fn FormatMessageW(
        dwFlags: DWORD,
        lpSource: LPCVOID,
        dwMessageId: DWORD,
        dwLanguageId: DWORD,
        lpBuffer: LPWSTR,
        nSize: DWORD,
        Arguments: va_list,
    ) -> DWORD;
    pub fn GetLastError() -> DWORD;
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;
    pub fn SetLastError(dwErrCode: DWORD);
}

#[link(name = "User32")]
extern "system" {
    pub fn BeginPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> HDC;
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
    pub fn DrawIcon(hDC: HDC, X: c_int, Y: c_int, hIcon: HICON) -> BOOL;
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
    pub fn EnumChildWindows(hWndParent: HWND, lpEnumFonc: ENUMCHILDPROC, lParam: LPARAM) -> BOOL;
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    pub fn GetMessageW(
        lpMsg: *const MSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
    ) -> BOOL;
    pub fn GetClassNameW(hWnd: HWND, lpClassName: LPWSTR, nMaxCount: c_int) -> c_int;
    pub fn GetClientRect(hWnd: HWND, lpRect: *const RECT) -> BOOL;
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
    pub fn LoadImageW(
        hInstance: HINSTANCE,
        name: LPCWSTR,
        r#type: UINT,
        cx: c_int,
        cy: c_int,
        fuLoad: UINT,
    ) -> HICON;
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> c_int;
    pub fn MoveWindow(
        hWnd: HWND,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        bRepaint: BOOL,
    ) -> BOOL;
    pub fn PostQuitMessage(nExitCode: c_int);
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
    pub fn SetWindowPos(
        hWnd: HWND,
        hWndInsertAfter: HWND,
        X: c_int,
        Y: c_int,
        cx: c_int,
        cy: c_int,
        uFlags: UINT,
    ) -> BOOL;
    pub fn SetWindowRgn(hWnd: HWND, hRgn: HRGN, bRedraw: BOOL) -> BOOL;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
}

pub fn get_class_name(hwnd: HWND) -> Result<String, Win32Error> {
    let mut class_name: Vec<u16> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    if unsafe { GetClassNameW(hwnd, class_name.as_mut_ptr(), 8) } == 0 {
        Err(get_last_error())
    } else {
        println!("{}", String::from_utf16(&class_name[..]).unwrap());
        Ok(String::from_utf16(&class_name[..]).unwrap())
    }
}

pub fn create_round_rect_rgn(
    x1: c_int,
    y1: c_int,
    x2: c_int,
    y2: c_int,
    w: c_int,
    h: c_int,
) -> Result<HRGN, ()> {
    let hrgn = unsafe { CreateRoundRectRgn(x1, y1, x2, y2, w, h) };
    if hrgn.is_null() {
        Err(())
    } else {
        Ok(hrgn)
    }
}

pub fn set_window_rgn(hwnd: HWND, hrgn: HRGN, bredraw: BOOL) -> Result<(), ()> {
    if unsafe { SetWindowRgn(hwnd, hrgn, bredraw) } != 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn set_window_pos(hwnd: HWND, x: i32, y: i32) -> Result<(), Win32Error> {
    if unsafe { SetWindowPos(hwnd, null_mut(), x, y, 0, 0, 0x0001 as u32) } != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

pub fn get_process_handle() -> HMODULE {
    unsafe { GetModuleHandleW(null()) }
}

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

pub fn register_class(window_class: &WNDCLASSW) -> Result<ATOM, Win32Error> {
    let atom = unsafe { RegisterClassW(window_class) };
    if atom == 0 {
        Err(get_last_error())
    } else {
        Ok(atom)
    }
}

pub fn get_client_rect(hwnd: HWND, lprect: *const RECT) -> Result<(), Win32Error> {
    if unsafe { GetClientRect(hwnd, lprect) } != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

pub fn create_app_window(
    class_name: &str,
    window_name: &str,
    position: Option<[i32; 2]>,
    [width, height]: [i32; 2],
    create_param: LPVOID,
) -> Result<HWND, Win32Error> {
    let class_name_null = wide_null(class_name);
    let window_name_null = wide_null(window_name);
    let (x, y) = match position {
        Some([x, y]) => (x, y),
        None => (CW_USEDEFAULT, CW_USEDEFAULT),
    };
    let hwnd = unsafe {
        CreateWindowExW(
            WS_EX_APPWINDOW,
            class_name_null.as_ptr(),
            window_name_null.as_ptr(),
            WS_CLIPCHILDREN | WS_CLIPSIBLINGS | WS_OVERLAPPEDWINDOW,
            x,
            y,
            width,
            height,
            null_mut(),
            null_mut(),
            get_process_handle(),
            create_param,
        )
    };
    if hwnd.is_null() {
        Err(get_last_error())
    } else {
        Ok(hwnd)
    }
}

pub fn create_custom_button(width: i32, height: i32, parent: HWND) -> Result<HWND, Win32Error> {
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            wide_null("button").as_ptr(),
            null_mut(),
            WS_CHILD | WS_VISIBLE | BS_OWNERDRAW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            parent,
            BTN_ID as HMENU,
            null_mut(),
            null_mut(),
        )
    };
    if hwnd.is_null() {
        Err(get_last_error())
    } else {
        Ok(hwnd)
    }
}

pub fn get_window_userdata<T>(hwnd: HWND) -> Result<*mut T, Win32Error> {
    set_last_error(Win32Error(0));
    let out = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) };
    if out == 0 {
        let last_error = get_last_error();
        if last_error.0 != 0 {
            Err(last_error)
        } else {
            Ok(out as *mut T)
        }
    } else {
        Ok(out as *mut T)
    }
}

pub fn get_window_id(hwnd: HWND) -> Result<isize, Win32Error> {
    let out = unsafe { GetWindowLongPtrW(hwnd, -12 as i32) };
    if out == 0 {
        Err(get_last_error())
    } else {
        Ok(out)
    }
}

pub fn set_window_userdata<T>(hwnd: HWND, ptr: *mut T) -> Result<*mut T, Win32Error> {
    set_last_error(Win32Error(0));
    let out = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr as LONG_PTR) };
    if out == 0 {
        let last_error = get_last_error();
        if last_error.0 != 0 {
            Err(last_error)
        } else {
            Ok(out as *mut T)
        }
    } else {
        Ok(out as *mut T)
    }
}

#[inline(always)]
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

pub fn post_quit_message(exit_code: c_int) {
    unsafe { PostQuitMessage(exit_code) }
}

pub fn move_window(
    hwnd: HWND,
    x: c_int,
    y: c_int,
    nwidth: c_int,
    nheight: c_int,
    brepaint: BOOL,
) -> Result<(), Win32Error> {
    if unsafe { MoveWindow(hwnd, x, y, nwidth, nheight, brepaint) } != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

pub fn HIWORD(l: DWORD) -> WORD {
    ((l >> 16) & 0xffff) as WORD
}

pub fn LOWORD(l: DWORD) -> WORD {
    (l & 0xffff) as WORD
}

pub fn begin_paint(hwnd: HWND) -> Result<(HDC, PAINTSTRUCT), Win32Error> {
    let mut ps = PAINTSTRUCT::default();
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
    if hdc.is_null() {
        Err(get_last_error())
    } else {
        Ok((hdc, ps))
    }
}

pub fn end_paint(hwnd: HWND, ps: &PAINTSTRUCT) {
    unsafe { EndPaint(hwnd, ps) };
}

pub fn fill_rect(hdc: HDC, rect: &RECT, hbr: HBRUSH) -> Result<(), ()> {
    if unsafe { FillRect(hdc, rect, hbr) } != 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn fill_rect_with_sys_color(hdc: HDC, rect: &RECT, color: SysColor) -> Result<(), ()> {
    if unsafe { FillRect(hdc, rect, (color as u32 + 1) as HBRUSH) } != 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn load_predefined_cursor(cursor: IDCursor) -> Result<HCURSOR, Win32Error> {
    let hcursor = unsafe { LoadCursorW(null_mut(), MAKEINTRESOURCEW(cursor as WORD)) };
    if hcursor.is_null() {
        Err(get_last_error())
    } else {
        Ok(hcursor)
    }
}

pub fn load_icon(source: &str) -> Result<HICON, Win32Error> {
    let hicon = unsafe {
        LoadImageW(
            null_mut(),
            wide_null(&format!("icon/{}.ico", source)).as_ptr(),
            IMAGE_ICON,
            0,
            0,
            LR_LOADFROMFILE,
        )
    };
    if hicon.is_null() {
        Err(get_last_error())
    } else {
        Ok(hicon)
    }
}

pub fn draw_icon(hdc: HDC, x: c_int, y: c_int, hicon: HICON) -> Result<(), Win32Error> {
    if unsafe { DrawIcon(hdc, x, y, hicon) } != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

pub fn set_last_error(e: Win32Error) {
    unsafe { SetLastError(e.0) }
}

pub fn get_last_error() -> Win32Error {
    Win32Error(unsafe { GetLastError() })
}

struct OnDropLocalFree(HLOCAL);
impl Drop for OnDropLocalFree {
    fn drop(&mut self) {
        unsafe { LocalFree(self.0) };
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Win32Error(pub DWORD);
impl std::error::Error for Win32Error {}
impl core::fmt::Display for Win32Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0 & (1 << 29) > 0 {
            return write!(f, "Win32ApplicationError({})", self.0);
        }
        pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x00000100;
        pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
        pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;
        let dwFlags = FORMAT_MESSAGE_ALLOCATE_BUFFER
            | FORMAT_MESSAGE_FROM_SYSTEM
            | FORMAT_MESSAGE_IGNORE_INSERTS;
        let lpSource = null_mut();
        let dwMessageId = self.0;
        let dwLanguageId = 0;
        let mut buffer: *mut u16 = null_mut();
        let lpBuffer = &mut buffer as *mut *mut u16 as *mut u16;
        let nSize = 0;
        let Arguments = null_mut();
        let tchar_count_excluding_null = unsafe {
            FormatMessageW(
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                Arguments,
            )
        };
        if tchar_count_excluding_null == 0 || buffer.is_null() {
            return Err(core::fmt::Error);
        } else {
            let _on_drop = OnDropLocalFree(buffer as HLOCAL);
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
