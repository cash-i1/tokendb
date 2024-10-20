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

        let response = match segments[..] {
            [""] => Response::from_string(root().into_string()).with_header(Header {
                field: "Content-Type".parse().unwrap(),
                value: "text/html".parse().unwrap(),
            }),
            ["assets", path] => {
                let file_path = Path::new("./assets/").join(path);
                let string = std::fs::read_to_string(file_path).unwrap();

                Response::from_string(string).with_header(Header {
                    field: "Content-Type".parse().unwrap(),
                    value: "text/plain".parse().unwrap(),
                })
            }
            _ => Response::from_string("".to_string()),
        };

        request.respond(response).unwrap();
    }
}

fn root() -> Markup {
    html!(
        h1 { "tokendb" }
    )
}
