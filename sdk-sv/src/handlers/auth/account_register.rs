use crate::{
    ArcState,
    app::{request::MyRegisterReq, response::MyRegisterRsp},
};
use axum::{
    extract::{Json, State},
    response::Html,
};
use db::sdk::user::User;

const REGISTER_PAGE: &'static str = r###"<html>
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
            width: 300px;
        }
        input, button {
            width: 100%;
            padding: 10px;
            margin: 5px 0;
            border-radius: 5px;
            background-color: #333;
            color: white;
            border: 1px solid #333;
        }
        button {
            background-color: #6200ea;
            cursor: pointer;
        }
        button:hover {
            background-color: #3700b3;
        }
        .password-container {
            display: flex;
            justify-content: space-between;
            align-items: center;
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
        <input type="text" id="username" name="username" placeholder="Username" required>
        
        <div class="password-container">
            <input type="password" id="password" name="password" placeholder="Password" required>
            <button type="button" onclick="togglePassword()">Show</button>
        </div>

        <button 
            hx-post="/account/register"
            hx-trigger="click" 
            hx-target="#response" 
            hx-swap="innerHTML"
            hx-include="#username, #password"
            hx-ext="json-enc">
            Register
        </button>
        <div id="response"></div>
    </div>

    <script>
        function togglePassword() {
            const passwordField = document.getElementById('password');
            const button = document.querySelector('.password-container button');
            if (passwordField.type === 'password') {
                passwordField.type = 'text';
                button.textContent = 'Hide';
            } else {
                passwordField.type = 'password';
                button.textContent = 'Show';
            }
        }
    </script>
</body>
</html>"###;

pub async fn get() -> Html<&'static str> {
    Html(REGISTER_PAGE)
}

pub async fn post(
    State(state): State<ArcState>,
    Json(req): Json<MyRegisterReq>,
) -> Html<MyRegisterRsp> {
    let username = &req.username;
    let password = &req.password;

    match User::validate_register_form(username, password) {
        Ok(_) => {}
        Err(e) => {
            return Html(format!("<p>{e}</p>"));
        }
    }

    match User::exists_by_username(&state.pool, username).await {
        Ok(true) => {
            return Html("<p>User Already Exists</p>".to_string());
        }
        Err(e) => {
            tracing::error!("{e}");
            // I'd love to return 500 status code, but HTMX is idk, kinda stupid
            // It wouldn't load the html unless it's 2xx code...
            return Html("<p>Server Error. Try Again Later.</p>".to_string());
        }
        _ => {}
    }

    let password_hash = match User::hash_password(password) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("{e}");
            return Html("<p>Server Error. Try Again Later.</p>".to_string());
        }
    };

    match User::create(&state.pool, username, &password_hash).await {
        Ok(v) => Html(format!("Successfully Registered With UID: {}", v)),
        Err(e) => {
            tracing::error!("{e}");
            Html("<p>Server Error. Try Again Later.</p>".to_string())
        }
    }
}
