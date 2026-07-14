//! CAP-N13 — point-in-time key state for the input overlay source.
//!
//! This is a **poll** of `GetAsyncKeyState`, deliberately NOT a keyboard
//! hook: there is no event queue, no buffer, and no ordering — each call
//! answers only "is this key down right now?" for the caller's fixed VK
//! set (which includes the mouse buttons — VK 0x01/0x02/0x04). Nothing is
//! logged or stored anywhere; the overlay samples only while its source
//! session is live.

use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

/// Whether the virtual key is down at this instant.
pub(crate) fn is_down(vk: i32) -> bool {
    // The high bit is "down right now". The low "pressed since the last
    // call" bit is deliberately ignored — reading it would make this a
    // (shared, racy) event consumer instead of a pure state peek.
    // SAFETY: plain global key-state read; no memory contract.
    let state = unsafe { GetAsyncKeyState(vk) } as u16;
    state & 0x8000 != 0
}

/// Whether each virtual key in `vks` is down at this instant, written into
/// `out` (cleared first) — the allocation-free form.
pub(crate) fn keys_down_into(vks: &[u16], out: &mut Vec<bool>) {
    out.clear();
    out.extend(vks.iter().map(|vk| is_down(i32::from(*vk))));
}

/// Whether each virtual key in `vks` is down at this instant.
pub(crate) fn keys_down(vks: &[u16]) -> Vec<bool> {
    let mut out = Vec::with_capacity(vks.len());
    keys_down_into(vks, &mut out);
    out
}
