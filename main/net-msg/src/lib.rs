pub use prost::Message as Trait;

#[macro_export]
macro_rules! dec {
    ($message:ident, $req:ident) => {
        $message::decode($req).expect(&format!("Failed to decode {}", stringify!($message)));
    };
}

pub mod pb {
    include!("../out/_.rs");
}

pub mod cmd {
    include!("../out/cmd.rs");
}
