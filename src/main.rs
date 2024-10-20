mod database;
mod html;
mod user;

use database::Database;
use user::User;

use std::path::Path;

use tiny_http::Header;
use tiny_http::Response;
use tiny_http::Server;

fn main() {
    let server = Server::http("0.0.0.0:2333").unwrap();
    let mut current_user: Option<User> = None;
    let database = Database::new(Path::new("./database.json"));

    for request in server.incoming_requests() {
        let path = request.url();
        let segments: Vec<&str> = path.trim_matches('/').split('/').collect();

        let response = match &segments[..] {
            [""] => Response::from_string(html::root(&mut current_user).into_string()).with_header(
                Header {
                    field: "Content-Type".parse().unwrap(),
                    value: "text/html".parse().unwrap(),
                },
            ),
            ["api", endpoint @ ..] => match endpoint {
                ["login", username, password] => {
                    let user = User::new(username.to_string(), password.to_string());
                    database.add_user_if_not_already_exists(&user);
                    current_user = Some(user.clone());

                    Response::from_string(user.token().to_string()).with_status_code(200)
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
