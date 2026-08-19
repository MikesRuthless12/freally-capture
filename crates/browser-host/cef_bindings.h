// Umbrella header for the CEF C API bindings the browser host drives (OSR).
// Kept minimal: bindgen pulls the transitive struct/enum defs from these.
#include "include/capi/cef_app_capi.h"
#include "include/capi/cef_client_capi.h"
#include "include/capi/cef_browser_capi.h"
#include "include/capi/cef_render_handler_capi.h"
#include "include/capi/cef_life_span_handler_capi.h"
#include "include/capi/cef_request_handler_capi.h"
#include "include/capi/cef_resource_request_handler_capi.h"
#include "include/internal/cef_types.h"

// The Windows sandbox handle factory. Chromium's renderer sandbox is the
// control that keeps a compromised web page from becoming code execution, and
// this host renders arbitrary http(s) URLs — so it is not optional. The two
// functions are plain `extern "C"`, but they live in cef_sandbox.lib (a static
// library built against the static CRT), which is why build.rs also switches
// this crate to +crt-static on Windows.
#ifdef _WIN32
#include "include/cef_sandbox_win.h"
#endif
