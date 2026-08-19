//! CAP-N78 — the injector, and the consent gate that guards it.
//!
//! Loading the hook DLL into a game is the single most consequential thing this
//! application does to another process, so the rules from
//! `design/game-hook-protocol.md` are enforced *here*, in code, not left to the
//! UI to remember:
//!
//! 1. **Per-executable opt-in.** [`inject`] refuses unless the caller passes a
//!    [`Consent`] naming the same executable it is about to inject into. The
//!    consent is minted only by [`Consent::grant`], which requires the caller to
//!    have shown [`crate::game::risk_warning`] and to pass the acknowledgement
//!    through — there is no "inject anything" entry point.
//! 2. **Never silently.** No scanning-and-attaching, no "hook all games", no
//!    background retry. One call, one named process, one user action.
//! 3. **Failure degrades.** A protected or refusing process returns an honest
//!    error and the caller uses WGC window capture.
//!
//! The mechanism is the ordinary documented one: `LoadLibraryW` on a remote
//! thread. `kernel32` is at the same base address in every process in a session,
//! so its `LoadLibraryW` address is valid cross-process; we write only the DLL
//! path into the target and start a thread at it. No shellcode, no patching of
//! game code, nothing hidden — the DLL is on disk and named.
//!
//! AUDITED `unsafe`: remote allocation, one write of a path string, and thread
//! creation. Nothing reads game memory.

use std::path::{Path, PathBuf};

use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0};
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
use windows::Win32::System::Memory::{
    VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE,
};
use windows::Win32::System::Threading::{
    CreateRemoteThread, GetExitCodeThread, OpenProcess, WaitForSingleObject,
    LPTHREAD_START_ROUTINE, PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION,
    PROCESS_VM_WRITE,
};

/// How long to wait for the remote `LoadLibraryW` to finish before giving up.
/// Generous — a game under load can be slow to schedule the thread — but bounded
/// so a wedged target can never hang the studio.
const LOAD_TIMEOUT_MS: u32 = 10_000;

/// Proof that the user was shown the anti-cheat risk and opted this specific
/// executable in.
///
/// It carries the executable name so [`inject`] can verify the consent matches
/// the process it is about to touch — consent for `game-a.exe` can never be
/// replayed against `game-b.exe`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Consent {
    executable: String,
}

impl Consent {
    /// Mint consent for one executable.
    ///
    /// `acknowledged` is the user's explicit "yes, I understand the ban risk"
    /// answer to [`crate::game::risk_warning`]. Anything falsy yields `None`,
    /// and without a `Consent` there is no way to reach [`inject`].
    pub fn grant(executable: &str, acknowledged: bool) -> Option<Self> {
        let executable = executable.trim();
        if !acknowledged || executable.is_empty() {
            return None;
        }
        Some(Self {
            // Windows executable names are case-insensitive; normalise once so
            // the match in `inject` cannot be defeated by casing.
            executable: executable.to_ascii_lowercase(),
        })
    }

    /// The executable this consent covers.
    pub fn executable(&self) -> &str {
        &self.executable
    }

    /// Does this consent authorise injecting into `executable`?
    pub fn covers(&self, executable: &str) -> bool {
        let name = Path::new(executable.trim())
            .file_name()
            .map(|s| s.to_string_lossy().to_ascii_lowercase())
            .unwrap_or_default();
        let mine = Path::new(&self.executable)
            .file_name()
            .map(|s| s.to_string_lossy().to_ascii_lowercase())
            .unwrap_or_default();
        !mine.is_empty() && mine == name
    }
}

/// Where the hook DLL ships: beside the app executable.
pub fn hook_dll_path() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|err| format!("could not locate the app: {err}"))?;
    let dir = exe
        .parent()
        .ok_or("the app executable has no parent directory")?;
    Ok(dir.join("freally_game_hook.dll"))
}

/// Inject the hook DLL into `pid`, which must be running `executable`.
///
/// Refuses without a [`Consent`] that covers `executable`. Returns an honest
/// error on any failure so the caller can fall back to WGC window capture.
pub fn inject(pid: u32, executable: &str, consent: &Consent) -> Result<(), String> {
    if !consent.covers(executable) {
        return Err(format!(
            "no Game Capture consent recorded for {executable} — enable it for this game first"
        ));
    }
    let dll = hook_dll_path()?;
    if !dll.is_file() {
        return Err(format!(
            "the game-capture hook is missing from this install ({})",
            dll.display()
        ));
    }
    inject_dll(pid, &dll)
}

/// The mechanical half, separated so [`inject`]'s policy is readable on its own.
fn inject_dll(pid: u32, dll: &Path) -> Result<(), String> {
    // The exact rights needed, nothing more — a protected/anti-cheat process
    // fails here, which is the honest "this title refuses the hook" answer.
    let access =
        PROCESS_CREATE_THREAD | PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_WRITE;
    // SAFETY: standard OpenProcess; the handle is closed on every path below.
    let process = unsafe { OpenProcess(access, false, pid) }.map_err(|err| {
        format!("this game will not allow Game Capture (it refused access): {err}")
    })?;

    let result = inject_into_open_process(process, dll);
    // SAFETY: `process` came from OpenProcess above and is closed exactly once.
    unsafe {
        let _ = CloseHandle(process);
    }
    result
}

fn inject_into_open_process(process: HANDLE, dll: &Path) -> Result<(), String> {
    // The DLL path as a NUL-terminated wide string — the only bytes we ever
    // write into the target.
    let mut wide: Vec<u16> = dll.as_os_str().encode_wide().collect();
    wide.push(0);
    let bytes = std::mem::size_of_val(&wide[..]);

    // SAFETY: reserve+commit a read/write page in the target for the path.
    let remote = unsafe {
        VirtualAllocEx(
            process,
            None,
            bytes,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        )
    };
    if remote.is_null() {
        return Err("could not reserve memory in the game process".into());
    }

    let (outcome, remote_is_idle) = write_and_run(process, remote, &wide, bytes);

    // Give the page back only once nothing in the game can still be reading it.
    // On the timeout path the remote `LoadLibraryW` may be mid-flight with this
    // exact pointer as its argument — releasing it there faults *inside the
    // game*, which the "a failure degrades, never crashes" rule forbids.
    // Leaking one path-sized page in a process we just failed to hook is by far
    // the cheaper mistake.
    if remote_is_idle {
        // SAFETY: `remote` came from VirtualAllocEx on this process handle, and
        // the remote thread that referenced it has exited.
        unsafe {
            let _ = VirtualFreeEx(process, remote, 0, MEM_RELEASE);
        }
    }
    outcome
}

/// Returns the outcome plus whether the remote page is safe to release — i.e.
/// whether it is known that no remote thread is still reading it.
fn write_and_run(
    process: HANDLE,
    remote: *mut std::ffi::c_void,
    wide: &[u16],
    bytes: usize,
) -> (Result<(), String>, bool) {
    use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;

    // Every failure before CreateRemoteThread leaves nothing in the game
    // referencing the page, so those paths are all safe to free.
    let mut written = 0usize;
    // SAFETY: `remote` is a committed RW region of exactly `bytes` in `process`.
    if let Err(err) = unsafe {
        WriteProcessMemory(
            process,
            remote,
            wide.as_ptr() as *const std::ffi::c_void,
            bytes,
            Some(&mut written),
        )
    } {
        return (
            Err(format!("could not hand the hook path to the game: {err}")),
            true,
        );
    }
    if written != bytes {
        return (
            Err("the hook path was only partly written to the game".into()),
            true,
        );
    }

    // kernel32 is mapped at the same address in every process of a session, so
    // our own LoadLibraryW address is valid in the target.
    // SAFETY: kernel32 is always loaded; the symbol always exists.
    let kernel32 = match unsafe { GetModuleHandleW(windows::core::w!("kernel32.dll")) } {
        Ok(handle) => handle,
        Err(err) => return (Err(format!("kernel32 is not loaded: {err}")), true),
    };
    let Some(load_library) =
        (unsafe { GetProcAddress(kernel32, windows::core::s!("LoadLibraryW")) })
    else {
        return (Err("LoadLibraryW is unavailable".into()), true);
    };
    // SAFETY: transmuting a real exported function to the thread-start ABI it
    // already matches (one pointer argument, DWORD return).
    let start: LPTHREAD_START_ROUTINE = Some(unsafe {
        std::mem::transmute::<
            unsafe extern "system" fn() -> isize,
            unsafe extern "system" fn(*mut std::ffi::c_void) -> u32,
        >(load_library)
    });

    // SAFETY: starts one thread at LoadLibraryW with the path we just wrote.
    let thread = match unsafe { CreateRemoteThread(process, None, 0, start, Some(remote), 0, None) }
    {
        Ok(thread) => thread,
        // The thread never started, so nothing is reading the page.
        Err(err) => {
            return (
                Err(format!("the game refused the capture thread: {err}")),
                true,
            )
        }
    };

    // SAFETY: waiting on the thread handle we just created.
    let waited = unsafe { WaitForSingleObject(thread, LOAD_TIMEOUT_MS) };
    let mut exit_code = 0u32;
    // SAFETY: reading the exit code of our own remote thread.
    let got_code = unsafe { GetExitCodeThread(thread, &mut exit_code) }.is_ok();
    // SAFETY: closed exactly once.
    unsafe {
        let _ = CloseHandle(thread);
    }

    // From here the page is safe to free only if the remote thread has exited.
    let remote_is_idle = waited == WAIT_OBJECT_0;
    if !remote_is_idle {
        return (
            Err("the game did not load the hook in time".into()),
            remote_is_idle,
        );
    }
    // LoadLibraryW returns the module handle; 0 means it failed to load.
    if got_code && exit_code == 0 {
        return (
            Err("the game loaded but rejected the hook (it may be protected)".into()),
            remote_is_idle,
        );
    }
    (Ok(()), remote_is_idle)
}

use std::os::windows::ffi::OsStrExt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consent_requires_an_explicit_acknowledgement() {
        // The whole point of the gate: no acknowledgement, no consent object,
        // and without a consent object there is no way to call inject().
        assert!(Consent::grant("game.exe", false).is_none());
        assert!(
            Consent::grant("", true).is_none(),
            "an empty name is not a game"
        );
        assert!(Consent::grant("   ", true).is_none());
        assert!(Consent::grant("game.exe", true).is_some());
    }

    #[test]
    fn consent_is_scoped_to_one_executable() {
        let consent = Consent::grant("MyGame.exe", true).expect("granted");
        assert!(consent.covers("MyGame.exe"));
        assert!(
            consent.covers("mygame.exe"),
            "Windows names are case-insensitive"
        );
        assert!(
            consent.covers(r"C:\Games\MyGame.exe"),
            "a full path still names the same executable"
        );
        // Consent for one game must never authorise another.
        assert!(!consent.covers("OtherGame.exe"));
        assert!(!consent.covers("mygame.exe.bak"));
        assert!(!consent.covers(""));
    }

    #[test]
    fn inject_refuses_without_matching_consent() {
        let consent = Consent::grant("MyGame.exe", true).expect("granted");
        // A real pid is irrelevant — the policy check happens before any
        // OpenProcess call, so a mismatched consent can never reach the target.
        let err = inject(u32::MAX, "SomeOtherGame.exe", &consent)
            .expect_err("consent does not cover this executable");
        assert!(
            err.contains("consent"),
            "the refusal must name the missing consent, got: {err}"
        );
    }
}
