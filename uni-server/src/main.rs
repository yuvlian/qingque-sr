use configs::logger::init_tracing;
use game_server::start_game_server;
use sdk_server::start_sdk_server;
use tokio::runtime::Builder;

fn main() {
    init_tracing();

    let game_sv_thread = std::thread::spawn(|| {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async { start_game_server().await }).unwrap();
    });

    let sdk_server_rt = Builder::new_current_thread().enable_all().build().unwrap();
    sdk_server_rt.block_on(async { start_sdk_server().await });

    let _ = game_sv_thread.join();
}
