#[cfg(windows)]
pub mod win32;

use win32::*;

use std::collections::HashMap;

const BN_POS: &str = "pos";
const BN_ICONS: &str = "icons";
const BN_INDEX: &str = "index";
const BN_STATE: &str =  "state";
const BN_ID: usize = 13;

#[derive(Debug)]
enum State {
    Idle,
    Down,
    Hover,
}

enum Params {
    Pos(((f32, i32), (f32, i32))),
    Icons(Vec<(Hicon, Hicon)>),
    Index(usize),
    State(State),
}
impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Btn<'a>(HashMap<&'a str, Params>);
impl Btn<'_> {
    pub fn new(inst: Hinstance, x: (f32, i32), y: (f32, i32), icons: Vec<(&str, &str)>) -> Self {
        let  mut data = HashMap::new();
        data.insert(BN_POS, Params::Pos((x, y)));
        data.insert(
            BN_ICONS,
            Params::Icons(
                icons
                    .into_iter()
                    .map(|(idle, hover)| {
                            (
                                load_icon(inst, idle).unwrap(),
                                load_icon(inst, hover).unwrap(),
                            )
                    })
                    .collect(),
            ),
        );
        data.insert(BN_INDEX, Params::Index(0));
        data.insert(BN_STATE, Params::State(State::Idle));
        Btn(data)
    }
    pub fn init(&mut self, parent: Hwnd, inst: Hinstance, width: i32, height: i32) -> Result<Hwnd, Win32Error> {
        let hwnd = create_control_window("button", width, height, parent, 0, inst)?;
        set_window_subclass(hwnd, Some(btn_proc), BN_ID).unwrap();
        for (key, val) in &mut self.0 {
            println!("{:?} {:?}", key, val);
            set_prop(hwnd, &key, val)?;
        }
        Ok(hwnd)
    }
}

pub unsafe extern "system" fn btn_proc(
    hwnd: Hwnd,
    msg: Uint,
    wparam: Wparam,
    lparam: Lparam,
    _uidsubclass: UintPtr,
    _dwrefdata: DwordPtr,
) -> Lresult {
    match msg {
        WM_NCDESTROY => {
            drop(RemovePropW(hwnd, wide_null(BN_STATE).as_ptr()));
            drop(RemovePropW(hwnd, wide_null(BN_POS).as_ptr()));
            drop(RemovePropW(hwnd, wide_null(BN_INDEX).as_ptr()));
            drop(RemovePropW(hwnd, wide_null(BN_ICONS).as_ptr()));
            return 0;
        }
        WM_PAINT => {
            match begin_paint(hwnd) {
                Ok((hdc, ps)) => {
                    let icons: &mut Vec<(Hicon, Hicon)> = get_prop(hwnd, BN_ICONS).unwrap();
                    let index: &mut usize = get_prop(hwnd, BN_INDEX).unwrap();
                    println!("{:?}", index);
                    match get_prop::<State>(hwnd, BN_STATE).unwrap() {
                        State::Idle => draw_icon(hdc, icons[*index].0).unwrap(),
                        State::Hover => draw_icon(hdc, icons[*index].1).unwrap(),
                        State::Down => {
                            if *index == icons.len() - 1 {
                                *index = 0
                            } else {
                                *index += 1
                            }
                            draw_icon(hdc, icons[*index].0).unwrap();
                        }
                    }
                    end_paint(hwnd, &ps);
                }
                Err(e) => println!("Couldn't begin painting: {}", e),
            }
            return 0;
        }
        WM_MOUSEMOVE => {
            let state: &mut State = get_prop(hwnd, BN_STATE).unwrap();
            *state = State::Hover;
            return DefSubclassProc(hwnd, msg, wparam, lparam);
        }
        WM_LBUTTONDOWN => {
            let state: &mut State = get_prop(hwnd, BN_STATE).unwrap();
            *state = State::Down;
            return DefSubclassProc(hwnd, msg, wparam, lparam);
        }
        WM_LBUTTONUP => {
            let state: &mut State = get_prop(hwnd, BN_STATE).unwrap();
            *state = State::Hover;
            return DefSubclassProc(hwnd, msg, wparam, lparam);
        }
        WM_MOUSELEAVE => {
            let state: &mut State = get_prop(hwnd, BN_STATE).unwrap();
            *state = State::Idle;
            return DefSubclassProc(hwnd, msg, wparam, lparam);
        }
        _ => DefSubclassProc(hwnd, msg, wparam, lparam),
    }
}
