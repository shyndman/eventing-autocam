use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

/// Initializes a tracing subscriber suitable for tests or examples
pub fn setup_dev_tracing_subscriber() {
    setup_dev_tracing_subscriber_with_env::<&str>(None);
}

pub fn setup_dev_tracing_subscriber_with_env<A: AsRef<str>>(maybe_env: Option<A>) {
    tracing_subscriber::registry()
        .with(fmt::layer().compact().with_file(false).without_time())
        .with(if let Some(env) = maybe_env {
            EnvFilter::from_env(env)
        } else {
            EnvFilter::from_default_env()
        })
        .init();
}