#![no_main]
//! `flz::decompress_into` writes through a cursor into a caller-owned slice and
//! trusts the input stream for every length and back-reference. It must be
//! total: any bytes at all must yield `Ok` or `Err`, never a panic and never a
//! write past the destination.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Split the input: the first byte picks a destination size so the fuzzer can
    // explore the "lying decoded length" axis as well as the stream itself.
    let Some((&size_byte, stream)) = data.split_first() else {
        return;
    };
    let out_len = usize::from(size_byte) * 64;
    let mut out = vec![0u8; out_len];
    let _ = fcap_encode::flz::decompress_into(stream, &mut out);
});
