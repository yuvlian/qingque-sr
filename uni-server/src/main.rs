use game_server::start_game_server;
use sdk_server::start_sdk_server;
use tokio::runtime::Builder;

fn main() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap_or(());

    tracing_subscriber::fmt().init();

    let game_sv_thread = std::thread::spawn(|| {
        // single thread
        let rt = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async { start_game_server().await }).unwrap();
    });

    let sdk_sv_thread = std::thread::spawn(|| {
        let rt = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async { start_sdk_server().await });
    });

    let _ = game_sv_thread.join();
    let _ = sdk_sv_thread.join();
}
