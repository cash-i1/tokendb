use std::path::Path;

use maud::html;
use maud::Markup;

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

use tiny_http::Header;
use tiny_http::Response;
use tiny_http::Server;

#[derive(Clone, Copy)]
struct Database {
    path: &'static Path,
}
impl Database {
    fn new(path: &'static Path) -> Database {
        if !path.is_file() {
            panic!()
        }
        if !std::fs::exists(path).unwrap() {
            std::fs::write(path, "").unwrap();
        }
        Database { path }
    }
    fn get_users(&self) -> Vec<User> {
        let string = std::fs::read_to_string(self.path).unwrap();
        let json: Value = serde_json::from_str(&string).unwrap();
        let users: Vec<User> = serde_json::from_value(json.get("users").unwrap().clone()).unwrap();
        users
    }
    fn get_user(&self, token: u64) -> Option<User> {
        let users = self.get_users();
        users.iter().cloned().find(|user| user.token == token)
    }
    fn add_user(&self, user: &User) {
        let string = std::fs::read_to_string(self.path).unwrap();
        let json: Value = serde_json::from_str(&string).unwrap();
        let mut users: Vec<User> =
            serde_json::from_value(json.get("users").unwrap().clone()).unwrap();
        users.push(user.clone());
        let new_json: Value = serde_json::from_value(json!({"users": users})).unwrap();
        let new_string = serde_json::to_string_pretty(&new_json).unwrap();
        std::fs::write(self.path, new_string).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    token: u64,
    username: String,
}

fn main() {
    let server = Server::http("0.0.0.0:2333").unwrap();
    let mut current_user: Option<User> = None;
    let database = Database::new(Path::new("./database.json"));

    for request in server.incoming_requests() {
        let path = request.url();
        let segments: Vec<&str> = path.trim_matches('/').split('/').collect();

        let response = match &segments[..] {
            [""] => {
                Response::from_string(root(&mut current_user).into_string()).with_header(Header {
                    field: "Content-Type".parse().unwrap(),
                    value: "text/html".parse().unwrap(),
                })
            }
            ["api", endpoint @ ..] => match endpoint {
                ["get_token", username, password] => {
                    let token = (username.chars().map(|c| c as u64).sum::<u64>()
                        * password.len() as u64)
                        << 2 * 4 + 42;
                    current_user = if let Some(user) = database.get_user(token) {
                        Some(user)
                    } else {
                        let new_user = User {
                            token,
                            username: username.to_string(),
                        };
                        database.add_user(&new_user);
                        Some(new_user)
                    };

                    Response::from_string(token.to_string()).with_status_code(200)
                }
                ["logout"] => {
                    current_user = None;
                    Response::from_string("logged out").with_status_code(200)
                }
                _ => Response::from_string("api endpoint does not exist").with_status_code(404),
            },
            ["assets", path] => {
                let file_path = Path::new("./assets/").join(path);
                let string = std::fs::read_to_string(file_path).unwrap();

                Response::from_string(string).with_header(Header {
                    field: "Content-Type".parse().unwrap(),
                    value: "text/plain".parse().unwrap(),
                })
            }
            _ => Response::from_string("page does not exist").with_status_code(404),
        };

        request.respond(response).unwrap();
    }
}

fn top_bar(user: &mut Option<User>) -> Markup {
    html!(
        div id="top_bar" {
            @if let Some(user) = user {
                span { "hello, " (user.username) "!" }

                button onclick="logout()" id="logout_button" { "logout" }
            } @else {
                span { "username" }
                input type="text" id="username_input" {}
                span { "password" }
                input type="text" id="password_input" {}

                button onclick="login()" id="login_button" { "login" }
            }
        }
        script {(maud::PreEscaped(
            r#"
            function login() {
                let username = document.getElementById("username_input").value;
                let password = document.getElementById("password_input").value;

                let token = fetch(`/api/get_token/${username}/${password}`)
                    .then(data => data.text())
                    .then(token => {
                        localStorage.setItem("token", token);
                        location.reload();
                    });

            }
            async function logout() {
                localStorage.clear();
                await fetch("/api/logout");
                location.reload();
            }
            "#))
        }
    )
}

fn root(user: &mut Option<User>) -> Markup {
    html!(
        (top_bar(user))
        h1 { "tokendb" }
    )
}
