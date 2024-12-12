pub use prost::Message as Trait;

pub mod pb {
    include!("../out/_.rs");
}

pub mod cmd {
    include!("../out/cmd.rs");
}
