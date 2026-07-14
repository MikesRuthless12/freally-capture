//! CAP-N13 — point-in-time key state for the input overlay source.
//!
//! This is a **poll** of `GetAsyncKeyState`, deliberately NOT a keyboard
//! hook: there is no event queue, no buffer, and no ordering — each call
//! answers only "is this key down right now?" for the caller's fixed VK
//! set (which includes the mouse buttons — VK 0x01/0x02/0x04). Nothing is
//! logged or stored anywhere; the overlay samples only while its source
//! session is live.

use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

/// Whether each virtual key in `vks` is down at this instant.
pub(crate) fn keys_down(vks: &[u16]) -> Vec<bool> {
    vks.iter()
        // The high bit is "down right now". The low "pressed since the last
        // call" bit is deliberately ignored — reading it would make this a
        // (shared, racy) event consumer instead of a pure state peek.
        .map(|vk| {
            let state = unsafe { GetAsyncKeyState(i32::from(*vk)) } as u16;
            state & 0x8000 != 0
        })
        .collect()
}
