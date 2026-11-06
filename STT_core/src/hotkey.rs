//! Global push-to-talk chord: Ctrl + Windows key (either side).
//! Uses `GetAsyncKeyState` so we can poll hold / release while recording.

#[cfg(windows)]
fn key_down(vk: i32) -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
    // High bit set => key is down right now.
    unsafe { GetAsyncKeyState(vk) as u16 & 0x8000 != 0 }
}

/// True while the user is holding the PTT chord (Ctrl and Win together).
#[cfg(windows)]
pub fn is_ptt_pressed() -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        VK_CONTROL, VK_LCONTROL, VK_LWIN, VK_RCONTROL, VK_RWIN,
    };

    // `windows` defines keys as `VIRTUAL_KEY`, a newtype around `u16` (not a plain integer).
    // `.0` unwraps that inner u16 (the Win32 virtual-key code). We cast to `i32` because
    // `GetAsyncKeyState` takes an `i32` in this crate's bindings.
    let ctrl = key_down(VK_CONTROL.0 as i32)
        || key_down(VK_LCONTROL.0 as i32)
        || key_down(VK_RCONTROL.0 as i32);
    let win = key_down(VK_LWIN.0 as i32) || key_down(VK_RWIN.0 as i32);
    ctrl && win
}

#[cfg(not(windows))]
pub fn is_ptt_pressed() -> bool {
    false
}
