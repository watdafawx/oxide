//! Live frame times from Windows' event tracing (ETW), the way PresentMon counts them.

#![allow(unused_imports)]

use ferrisetw::native::EvntraceNativeError;
use ferrisetw::provider::Provider;
use ferrisetw::trace::{TraceError, TraceTrait};
use ferrisetw::{EventRecord, SchemaLocator, UserTrace};

/// Why live frame counting is not running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveError {
    /// Event tracing needs administrator.
    NeedsAdmin,
    /// Anything else, in Windows' words.
    Failed(String),
}

/// Turn ferrisetw's error into a reason the overlay can show.
pub fn live_error(error: TraceError) -> LiveError {
    match error {
        TraceError::EtwNativeError(EvntraceNativeError::IoError(e)) if e.raw_os_error() == Some(5) => LiveError::NeedsAdmin,
        other => LiveError::Failed(format!("{other:?}")),
    }
}

/// Microsoft-Windows-DXGI: every Direct3D 10/11/12 present goes through it.
pub const DXGI_PROVIDER: &str = "ca11c036-0102-4a2d-a6ad-f03cfed5d3c9";
/// DXGI's Present_Start event.
pub const PRESENT_START: u16 = 42;

/// Start listening for presents. `on_present` gets the process id and the time in seconds.
/// Keep the returned trace alive: dropping it stops the session.
pub fn start_present_trace(on_present: impl Fn(u32, f64) + Send + Sync + 'static) -> Result<UserTrace, LiveError> {
    let provider = Provider::by_guid(DXGI_PROVIDER)
        .add_callback(move |record: &EventRecord, _: &SchemaLocator| {
            if record.event_id() == PRESENT_START {
                // raw_timestamp is in 100-nanosecond steps.
                on_present(record.process_id(), record.raw_timestamp() as f64 / 10_000_000.0);
            }
        })
        .build();
    UserTrace::new().named("KinetikPresents".to_string()).enable(provider).start_and_process().map_err(live_error)
}
