use std::path::Path;

use maud::html;
use maud::Markup;

use tiny_http::Header;
use tiny_http::Response;
use tiny_http::Server;

fn main() {
    let server = Server::http("0.0.0.0:2333").unwrap();

    for request in server.incoming_requests() {
        let path = request.url();
        let segments: Vec<&str> = path.trim_matches('/').split('/').collect();

        let response = match &segments[..] {
            [""] => Response::from_string(root().into_string()).with_header(Header {
                field: "Content-Type".parse().unwrap(),
                value: "text/html".parse().unwrap(),
            }),
            ["api", endpoint @ ..] => match endpoint {
                ["login", username, password] => {
                    let token = (username.chars().map(|c| c as usize).sum::<usize>()
                        * password.len())
                        << 2 * 4 + 42;
                    Response::from_string(token.to_string()).with_status_code(200)
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

fn top_bar() -> Markup {
    html!(
        div id="top_bar" {
            span { "username" }
            input type="text" id="username_input" {}
            span { "password" }
            input type="text" id="password_input" {}

            button onclick="login()" id="login_button" { "login" }
        }
        script {(maud::PreEscaped(
            r#"
            function login() {
                let username = document.getElementById("username_input").value;
                let password = document.getElementById("password_input").value;

                let token = fetch(`/api/login/${username}/${password}`)
                    .then(data => data.text())
                    .then(token => {
                        localStorage.setItem("token", token);
                    });

            }
            "#))
        }
    )
}

fn root() -> Markup {
    html!(
        (top_bar())
        h1 { "tokendb" }
    )
}
