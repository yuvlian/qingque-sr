CREATE TABLE gateway_hotfix (
    game_version TEXT PRIMARY KEY,
    asset_bundle_url TEXT NOT NULL,
    ex_resource_url TEXT NOT NULL,
    lua_url TEXT NOT NULL,
    ifix_url TEXT NOT NULL
);

CREATE TABLE user (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    -- is_banned INTEGER NOT NULL DEFAULT 0 CHECK (is_banned IN (0, 1)),
    user_token TEXT,
    register_date TEXT NOT NULL
)
