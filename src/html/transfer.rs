use crate::database::Database;
use crate::user::User;
use maud::html;
use maud::Markup;

pub fn transfer(database: &mut Database) -> Markup {
    html!(
        (super::head())
        (super::top_bar(database))
        h1 { "transfer" }
        @if let Some(current_user) = &database.current_user {
            span { "current balance: " (current_user.balance) }
            br {}

            span { "transfer " }
            input type="number" id="amount_input" {}
            span { " tokens to " }

            select id="to_select" {
                @for user in database.get_users() {
                    @if database.current_user.is_some()
                    && database.current_user.clone().unwrap().token() == user.token() {
                        continue;
                    }
                    option value=(user.token()) { (user.username) }
                }
            }
            br {}
            button disabled[database.current_user.is_none()] onclick="transfer()" { "transfer" }
        } @else {
            h2 { "log in to transfer" }
        }

        script {(maud::PreEscaped(
            r#"
            async function transfer() {
                console.log("asdfasdfasdfasdf");
                let from = localStorage.getItem("token");
                let to = document.getElementById("to_select").value;
                let amount = document.getElementById("amount_input").value;

                let resp = await fetch(`/api/transfer/${from}/${to}/${amount}`);
                console.log(resp);

                location.reload();
            }
            "#))
        }
    )
}
