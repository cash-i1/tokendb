use crate::database::Database;
use maud::html;
use maud::Markup;

pub fn balance(database: &mut Database) -> Markup {
    html!(
        (super::top_bar(database))
        h1 { "balance" }
        @if let Some(current_user) = &database.current_user {
            span { "your balance is " strong {(current_user.balance)} " tokens" }
        } @else {
            span { "login to see your balance" }
        }
    )
}
