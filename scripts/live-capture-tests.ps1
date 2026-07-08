# Runs each fcap-capture live-hardware test in its OWN process.
#
# Why: the OS capture stacks (WinRT/D3D) tear down racily when several capture
# sessions run inside one test process — multi-test runs intermittently
# ACCESS_VIOLATION the harness even with --test-threads=1, while every test
# passes alone (see the header of crates/capture/tests/live_capture.rs).
# Process-per-test is the isolation that actually holds: each session's
# teardown finishes when its process exits, so nothing races.
#
# Usage:  .\scripts\live-capture-tests.ps1            # every live test
#         .\scripts\live-capture-tests.ps1 cursor     # only tests matching *cursor*
#
# Needs a real display session (not headless CI); the cursor tests move the
# mouse and spawn throwaway console windows for a few seconds.

param([string]$Filter = "")

# Build the test binary once so the per-test runs don't each pay the compile.
cargo build -p fcap-capture --tests
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

# Enumerate the ignored (= live) tests.
$listing = cargo test -p fcap-capture --test live_capture -- --list --ignored
if ($LASTEXITCODE -ne 0) { Write-Error "listing the live tests failed"; exit 1 }
$tests = $listing | Where-Object { $_ -match '^(\S+): test$' } | ForEach-Object { $_ -replace ': test$', '' }
if ($Filter) { $tests = @($tests | Where-Object { $_ -like "*$Filter*" }) }
if (-not $tests) { Write-Error "no live tests matched '$Filter'"; exit 1 }

$failed = @()
foreach ($test in $tests) {
    Write-Host ""
    Write-Host "=== $test ===" -ForegroundColor Cyan
    cargo test -p fcap-capture --test live_capture -- --ignored --exact $test
    if ($LASTEXITCODE -ne 0) { $failed += $test }
}

Write-Host ""
if ($failed) {
    Write-Host "FAILED ($($failed.Count)/$($tests.Count)): $($failed -join ', ')" -ForegroundColor Red
    exit 1
}
Write-Host "all $(@($tests).Count) live tests passed, one process each" -ForegroundColor Green
