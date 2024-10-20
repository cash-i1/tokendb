use crate::user::User;
use maud::html;
use maud::Markup;

pub fn top_bar(user: &mut Option<User>) -> Markup {
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
