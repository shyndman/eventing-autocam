pub mod color;
#[allow(unused_imports)]
pub mod debug;
pub mod geom;
pub mod gst;
pub mod path;

use ::gst::{DebugCategory, DebugColorFlags};
use once_cell::sync::Lazy;

pub(self) static CAT: Lazy<DebugCategory> = Lazy::new(|| {
    DebugCategory::new(
        "AA_UTIL",
        DebugColorFlags::empty(),
        Some("Auto-Arena Utilities"),
    )
});