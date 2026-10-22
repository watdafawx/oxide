//! Live frame times from Windows' event tracing (ETW), the way PresentMon counts them.

#![allow(unused_imports)]

#[allow(unused_imports)]
use ferrisetw::native::EvntraceNativeError;
#[allow(unused_imports)]
use ferrisetw::provider::Provider;
#[allow(unused_imports)]
use ferrisetw::trace::{TraceError, TraceTrait};
#[allow(unused_imports)]
use ferrisetw::{EventRecord, SchemaLocator, UserTrace};

/// Why live frame counting is not running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveError {
    /// Event tracing needs administrator.
    NeedsAdmin,
    /// Anything else, in Windows' words.
    Failed(String),
}

