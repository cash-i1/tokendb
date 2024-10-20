use crate::database::Database;
use maud::html;
use maud::Markup;

pub fn account(database: &mut Database) -> Markup {
    html!(
        (super::head())
        (super::top_bar(database))
        h1 { "account" }
        @if database.current_user.is_none() {
            span { "you are not logged in" }

            br {}

            span { "username" }
            input type="text" id="username_input" {}
            span { "password" }
            input type="text" id="password_input" {}
            button onclick="login()" id="login_button" { "login" }
        } @else {
            button onclick="logout()" id="logout_button" { "logout" }
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
