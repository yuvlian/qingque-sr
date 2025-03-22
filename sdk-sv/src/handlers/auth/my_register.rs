use crate::{
    ArcState,
    app::{db::user::User, request::MyRegisterReq, response::MyRegisterRsp},
};
use axum::{
    extract::{Json, State},
    response::Html,
};
use utils::hash_password;

pub async fn get() -> Html<&'static str> {
    Html(
        r###"<html>
<head>
    <script src="https://unpkg.com/htmx.org@2.0.4"></script>
    <script src="https://unpkg.com/htmx-ext-json-enc@2.0.1/json-enc.js"></script>
    <style>
        body {
            background-color: #121212;
            color: #e0e0e0;
            font-family: Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
        }
        .container {
            text-align: center;
            background-color: #1e1e1e;
            padding: 20px;
            border-radius: 10px;
            box-shadow: 0px 4px 10px rgba(255, 255, 255, 0.1);
        }
        h1 {
            margin-bottom: 20px;
        }
        input {
            width: 100%;
            padding: 10px;
            margin: 5px 0;
            border: none;
            border-radius: 5px;
            background-color: #333;
            color: white;
        }
        button {
            width: 100%;
            padding: 10px;
            margin-top: 10px;
            border: none;
            border-radius: 5px;
            background-color: #6200ea;
            color: white;
            cursor: pointer;
        }
        button:hover {
            background-color: #3700b3;
        }
        #response {
            margin-top: 10px;
            padding: 10px;
            background-color: #292929;
            border-radius: 5px;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>stalel</h1>
        <input type="text" id="username" name="username" placeholder="Username" required>
        <input type="password" id="password" name="password" placeholder="Password" required>
        <button 
            hx-post="/my_register"
            hx-trigger="click" 
            hx-target="#response" 
            hx-swap="innerHTML"
            hx-include="#username, #password"
            hx-ext="json-enc">
            Register
        </button>
        <div id="response"></div>
    </div>
</body>
</html>"###,
    )
}

pub async fn post(
    State(state): State<ArcState>,
    Json(req): Json<MyRegisterReq>,
) -> Html<MyRegisterRsp> {
    let pass_len = req.password.len();
    let usrn_len = req.username.len();

    if pass_len < 6 || pass_len > 72 {
        return Html("<p>Password Length Must Be Between 6 and 72</p>".into());
    }

    if usrn_len < 4 || usrn_len > 16 {
        return Html("<p>Username Length Must Be Between 4 and 16</p>".into());
    }

    match User::exists_by_username(&state.pool, &req.username).await {
        Ok(true) => {
            return Html("<p>User Already Exists</p>".into());
        }
        Err(e) => {
            tracing::error!("{e}");
            // I'd love to return 500 status code, but HTMX is idk, kinda stupid
            // It wouldn't load the html unless it's 2xx code...
            return Html("<p>Server Error. Try Again Later.</p>".into());
        }
        _ => {}
    }

    let password_hash = match hash_password(&req.password) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("{e}");
            return Html("<p>Server Error. Try Again Later.</p>".into());
        }
    };

    match User::create(&state.pool, &req.username, &password_hash).await {
        Ok(v) => Html(format!("Successfully Registered With UID: {}", v)),
        Err(e) => {
            tracing::error!("{e}");
            Html("<p>Server Error. Try Again Later.</p>".into())
        }
    }
}
