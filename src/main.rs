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
    let mut database = Database::new(Path::new("./database.json"));

    for request in server.incoming_requests() {
        let path = request.url();
        let segments: Vec<&str> = path.trim_matches('/').split('/').collect();

        let response = match &segments[..] {
            [""] => {
                Response::from_string(html::root(&mut database).into_string()).with_header(Header {
                    field: "Content-Type".parse().unwrap(),
                    value: "text/html".parse().unwrap(),
                })
            }
            ["transfer"] => Response::from_string(html::transfer(&mut database).into_string())
                .with_header(Header {
                    field: "Content-Type".parse().unwrap(),
                    value: "text/html".parse().unwrap(),
                }),
            ["balance"] => Response::from_string(html::balance(&mut database).into_string())
                .with_header(Header {
                    field: "Content-Type".parse().unwrap(),
                    value: "text/html".parse().unwrap(),
                }),
            ["account"] => Response::from_string(html::account(&mut database).into_string())
                .with_header(Header {
                    field: "Content-Type".parse().unwrap(),
                    value: "text/html".parse().unwrap(),
                }),
            ["api", endpoint @ ..] => match endpoint {
                ["login", username, password] => {
                    let user = User::new(username.to_string(), password.to_string());
                    database.add_user_if_not_already_exists(&user);
                    database.current_user = Some(user.clone());

                    Response::from_string(user.token().to_string()).with_status_code(200)
                }
                ["logout"] => {
                    database.current_user = None;
                    Response::from_string("logged out").with_status_code(200)
                }
                ["i_have_token", token] => {
                    let token = token.parse::<u64>();
                    if token.is_err() {
                        Response::from_string("invalid token").with_status_code(404)
                    } else {
                        let token = token.unwrap();

                        let user = database.get_user(token);
                        if user.is_none() {
                            Response::from_string(
                                "token does not belong to an already created user",
                            )
                            .with_status_code(404)
                        } else {
                            let user = user.unwrap();

                            database.current_user = Some(user);
                            Response::from_string("logged in").with_status_code(200)
                        }
                    }
                }
                ["transfer", from, to, amount] => {
                    let from = from.parse::<u64>();
                    let to = to.parse::<u64>();

                    if from.is_err() || to.is_err() {
                        Response::from_string("invalid tokens").with_status_code(404)
                    } else {
                        let from = from.unwrap();
                        let to = to.unwrap();

                        let from_user = database.get_user(from);
                        let to_user = database.get_user(to);

                        if from_user.is_none() || to_user.is_none() {
                            Response::from_string("users dont exist").with_status_code(404)
                        } else {
                            let mut from_user = from_user.unwrap();
                            let mut to_user = to_user.unwrap();

                            from_user.balance -= amount.parse::<f32>().unwrap();
                            to_user.balance += amount.parse::<f32>().unwrap();
                            
                            println!("fub: {}, tub: {}", from_user.balance, to_user.balance);

                            database.update_user(from, &from_user);
                            database.update_user(to, &from_user);

                            Response::from_string("transfered").with_status_code(200)
                        }
                    }
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
