//! Small OS signals the automation engine's triggers read (CAP-N01):
//! how long the user has been idle, and which app owns the foreground window.
//!
//! Windows-first and honest about it: elsewhere these return `None`, the
//! matching triggers simply never fire, and the UI says so instead of
//! pretending. Read-only — nothing here changes system state, and neither
//! signal ever leaves the machine.

/// Seconds since the last keyboard/mouse input, when the OS will say.
pub fn idle_seconds() -> Option<u32> {
    #[cfg(target_os = "windows")]
    {
        crate::win::idle_seconds()
    }
    #[cfg(not(target_os = "windows"))]
    {
        None
    }
}

/// The foreground window's exe file name, when the OS will say.
pub fn foreground_exe() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        crate::win::foreground_exe()
    }
    #[cfg(not(target_os = "windows"))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_signals_are_readable_or_honestly_absent() {
        // Both may be None (non-Windows, or a locked-down session); what must
        // never happen is a panic or a hang.
        let idle = idle_seconds();
        assert!(idle.is_none() || idle.is_some_and(|seconds| seconds < 60 * 60 * 24 * 30));
        let exe = foreground_exe();
        assert!(exe.is_none() || exe.is_some_and(|name| !name.is_empty()));
    }
}
