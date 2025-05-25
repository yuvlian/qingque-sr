use std::time::{SystemTime, UNIX_EPOCH};

pub fn cur_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_millis() as u64
}

pub fn cur_timestamp_for_seed() -> u32 {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs()
        / 5)
    .try_into()
    .expect("Timestamp integer overflow")
}

pub mod chat {
    pub const PLAYER_UID: u32 = 800047331;
    // pub const FRIEND_UID: u32 = 727;
    // pub const FRIEND_ICON_ID: u32 = 201402;
    // pub const FRIEND_CHAT_BUBBLE_ID: u32 = 220005;
    // const fn reverse_friend_chat_history() -> [&'static str; 5] {
    //     let input = [
    //         "Available commands:",
    //         "'lineup [Vec<u32>]' to change lineup. ExampleUsage: lineup [1001,1002,1003,1004]. AVATAR ID MUST BE IN OWNED AVATARS.",
    //         "'march [march_element]' to change march multipath. Element:Ice,Imaginary. ExampleUsage: march ice",
    //         "'mc [mc_gender] [mc_element]' to change mc multipath. Gender:Man,Woman. Element:Physical,Fire,Imaginary,Ice. ExampleUsage: mc man ice",
    //         "NOTE: USING THESE COMMANDS WON'T CHANGE THE avatar.toml, MEANING ONLY THE CURRENT SESSION WILL BE AFFECTED",
    //     ];
    //     [input[4], input[3], input[2], input[1], input[0]]
    // }
    // pub const FRIEND_CHAT_HISTORY: [&str; 5] = reverse_friend_chat_history();
}
