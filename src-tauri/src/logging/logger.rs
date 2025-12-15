// Structured logging with tracing

pub struct SmoothieLogger;

impl SmoothieLogger {
  /// Initialize logging system with file and console output
  pub fn init() {
    tracing_subscriber::fmt()
      .with_env_filter(
        tracing_subscriber::EnvFilter::from_default_env()
          .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
      )
      .with_file(true)
      .with_line_number(true)
      .with_thread_ids(true)
      .with_target(true)
      .init();

    tracing::info!("Smoothie logging initialized");
  }
}
