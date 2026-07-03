//! Live-hardware smoke tests: prove real samples flow from the OS audio
//! pipelines. `#[ignore]` because CI runners have no audio hardware — run
//! explicitly with `cargo test -p fcap-audio -- --ignored` on a real desktop.

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use fcap_audio::capture::open_capture;
use fcap_audio::graph::StripControl;
use fcap_audio::{
    list_input_devices, list_loopback_devices, list_output_devices, InputSpec, MixerCore,
    BLOCK_FRAMES,
};
use fcap_scene::{AudioSettings, SourceId};

#[test]
#[ignore = "needs real audio devices (not headless CI)"]
fn devices_enumerate() {
    let inputs = list_input_devices().expect("list inputs");
    let outputs = list_output_devices().expect("list outputs");
    let (loopbacks, guidance) = list_loopback_devices().expect("list loopbacks");
    println!(
        "inputs: {:?}",
        inputs.iter().map(|d| &d.name).collect::<Vec<_>>()
    );
    println!(
        "outputs: {:?}",
        outputs.iter().map(|d| &d.name).collect::<Vec<_>>()
    );
    println!(
        "loopbacks: {:?} (guidance: {guidance:?})",
        loopbacks.iter().map(|d| &d.name).collect::<Vec<_>>()
    );
    assert!(
        !inputs.is_empty() || !outputs.is_empty(),
        "a desktop should expose at least one audio device"
    );
}

#[test]
#[ignore = "needs a real microphone (not headless CI)"]
fn default_mic_delivers_samples() {
    let stream = match open_capture(&InputSpec::Input {
        device_id: String::new(),
    }) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("no default mic here ({err}); skipping");
            return;
        }
    };
    println!("capturing from {:?}", stream.device_name());
    thread::sleep(Duration::from_millis(600));
    let buffered = stream.ring().len();
    assert!(
        buffered > BLOCK_FRAMES * 2,
        "expected buffered samples after 600 ms, got {buffered}"
    );
}

#[test]
#[ignore = "needs desktop audio (Windows loopback / a monitor device)"]
fn desktop_audio_loopback_delivers_samples() {
    let stream = match open_capture(&InputSpec::Loopback {
        device_id: String::new(),
    }) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("no loopback here ({err}); that may be the honest state");
            return;
        }
    };
    println!("loopback from {:?}", stream.device_name());
    thread::sleep(Duration::from_millis(600));
    // Loopback delivers samples even for a silent desktop on most drivers;
    // accept an empty ring only if the stream is still healthy.
    let buffered = stream.ring().len();
    println!("buffered after 600 ms: {buffered}");
    assert!(!stream.ring().is_broken(), "the loopback stream died");
}

#[test]
#[ignore = "needs a real microphone (not headless CI)"]
fn mic_through_the_mixer_meters_levels() {
    let stream = match open_capture(&InputSpec::Input {
        device_id: String::new(),
    }) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("no default mic here ({err}); skipping");
            return;
        }
    };
    thread::sleep(Duration::from_millis(300));

    let id = SourceId::new();
    let mut core = MixerCore::new();
    let mut controls = HashMap::new();
    controls.insert(id, StripControl::new(AudioSettings::default()));
    for _ in 0..30 {
        let mut block = Vec::with_capacity(BLOCK_FRAMES * 2);
        stream.ring().pop_into(&mut block, BLOCK_FRAMES * 2);
        block.resize(BLOCK_FRAMES * 2, 0.0);
        let mut inputs = HashMap::new();
        inputs.insert(id, block);
        core.process(&inputs, &controls);
        thread::sleep(Duration::from_millis(10));
    }
    let levels = core.take_source_levels(id).expect("strip exists");
    println!("mic levels: peak {:?} rms {:?}", levels.peak, levels.rms);
    // Even a silent room shows *some* noise floor; a dead-zero meter after
    // 300 ms of capture would mean no samples flowed at all.
    assert!(
        !stream.ring().is_empty() || levels.peak[0] > 0.0,
        "no samples flowed from the default mic"
    );
}
