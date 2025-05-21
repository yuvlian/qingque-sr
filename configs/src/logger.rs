pub fn init_tracing() {
    tracing_subscriber::fmt().without_time().init();

    #[cfg(target_os = "windows")]
    if let Err(e) = ansi_term::enable_ansi_support() {
        tracing::warn!("failed enabling ansi: {}", e);
    }
}
