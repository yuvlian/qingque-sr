use game_server::start_game_server;
use sdk_server::start_sdk_server;
use tokio::runtime::Builder;

fn main() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().expect("failed to enable ansi");

    tracing_subscriber::fmt().init();

    let game_sv_thread = std::thread::spawn(|| {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async { start_game_server().await }).unwrap();
    });

    let sdk_server_rt = Builder::new_current_thread().enable_all().build().unwrap();
    sdk_server_rt.block_on(async { start_sdk_server().await });

    let _ = game_sv_thread.join();
}
