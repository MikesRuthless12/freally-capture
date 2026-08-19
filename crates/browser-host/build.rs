//! Build script for the browser host.
//!
//! Without the `cef` feature this is a no-op — the skeleton host needs no SDK.
//! With `cef`, it binds the CEF **C API** headers (bindgen, so struct layouts
//! are exact, never hand-transcribed) and links `libcef` from the SDK at
//! `CEF_ROOT`. The SDK is the on-demand component, never vendored into the repo.

fn main() {
    #[cfg(feature = "cef")]
    cef::build();
}

#[cfg(feature = "cef")]
mod cef {
    use std::env;
    use std::path::PathBuf;

    /// CEF 127+ versions its C ABI: the struct layouts (and libcef's ctocpp
    /// validation) depend on this. It is stated ONCE here — bindgen is given it
    /// as a define, and the same value is written into `OUT_DIR` for the
    /// backend to include, so the layout the bindings assume and the version
    /// the backend hands `cef_api_hash` cannot drift. A mismatch is not a
    /// compile error: libcef reads every struct as version -1 and aborts.
    const CEF_API_VERSION: u32 = 13300;

    pub fn build() {
        // The staged CEF SDK (headers + Release/libcef.lib). The component build
        // exports this; locally it points at the fetched, hash-verified SDK.
        let root = env::var("CEF_ROOT").unwrap_or_else(|_| {
            panic!(
                "CEF_ROOT is not set. Build the `cef` feature against the pinned CEF SDK: \
                 set CEF_ROOT to the extracted cef_binary_… directory."
            )
        });
        let root = PathBuf::from(root);
        let include = root.join("include");
        let capi = include.join("capi");
        assert!(
            capi.join("cef_app_capi.h").is_file(),
            "CEF_ROOT/{} does not look like a CEF SDK (no include/capi/cef_app_capi.h)",
            root.display()
        );

        // Link libcef from the SDK's Release dir. The C API exports resolve
        // against libcef.lib (the import lib for libcef.dll).
        println!(
            "cargo:rustc-link-search=native={}",
            root.join("Release").display()
        );
        println!("cargo:rustc-link-lib=dylib=libcef");

        // The Windows renderer sandbox. `cef_sandbox_info_create` lives in
        // cef_sandbox.lib, a STATIC library CEF builds against the static CRT
        // (/MT). Rust's MSVC target defaults to the dynamic CRT (/MD), and
        // mixing the two is a link error — so the host must be built with
        // `-C target-feature=+crt-static`, which the component workflow sets
        // for this crate's own cargo invocation. libcef itself is a DLL with
        // its own CRT, so it is unaffected either way.
        if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
            println!("cargo:rustc-link-lib=static=cef_sandbox");
            // cef_sandbox.lib's own dependencies.
            for lib in [
                "Advapi32",
                "dbghelp",
                "Delayimp",
                "ntdll",
                "OleAut32",
                "PowrProf",
                "Propsys",
                "psapi",
                "SetupAPI",
                "Shell32",
                "Shlwapi",
                "Userenv",
                "version",
                "wbemuuid",
                "WindowsApp",
                "winmm",
            ] {
                println!("cargo:rustc-link-lib=dylib={lib}");
            }
        }

        println!("cargo:rerun-if-env-changed=CEF_ROOT");
        println!("cargo:rerun-if-changed=cef_bindings.h");

        // Generate the bindings for exactly the C API we drive. A small umbrella
        // header (checked into the crate) includes just what OSR needs; bindgen
        // pulls in the transitive struct/enum definitions from there.
        let bindings = bindgen::Builder::default()
            .header("cef_bindings.h")
            .clang_arg(format!("-I{}", include.parent().unwrap().display()))
            .clang_arg(format!("-I{}", include.display()))
            // The C API only — no C++.
            .clang_arg("-DWIN32")
            // Pin the SDK's documented baseline so bindgen lays out exactly
            // what this libcef expects — without it the headers fall back to
            // the experimental layout.
            .clang_arg(format!("-DCEF_API_VERSION={CEF_API_VERSION}"))
            .allowlist_function("cef_.*")
            .allowlist_type("cef_.*")
            .allowlist_var("CEF_.*")
            .derive_default(true)
            .layout_tests(false)
            .generate()
            .expect("bindgen could not generate the CEF C API bindings");

        let out = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out.join("cef_bindings.rs"))
            .expect("could not write cef_bindings.rs");

        // The one definition the backend includes, so it cannot state a
        // different version than the bindings were generated against.
        std::fs::write(
            out.join("cef_api_version.rs"),
            format!("const CEF_API_VERSION: i32 = {CEF_API_VERSION};\n"),
        )
        .expect("could not write cef_api_version.rs");
    }
}
