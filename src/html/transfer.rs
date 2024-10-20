use crate::database::Database;
use crate::user::User;
use maud::html;
use maud::Markup;

pub fn transfer(database: &mut Database) -> Markup {
    html!(
        (super::head())
        (super::top_bar(database))
        h1 { "transfer" }
        @if database.current_user.is_some() {
            span { "transfer " }
            input type="number" {}
            span { " tokens to " }

            select {
                @for user in database.get_users() {
                    @if database.current_user.is_some()
                    && database.current_user.clone().unwrap().token() == user.token() {
                        continue;
                    }
                    option value=(user.token()) { (user.username) }
                }
            }
            br {}
            button disabled[database.current_user.is_none()] { "transfer" }
        } @else {
            h2 { "log in to transfer" }
        }
    )
}
