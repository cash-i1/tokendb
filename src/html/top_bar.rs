use crate::database::Database;
use maud::html;
use maud::Markup;

pub fn top_bar(database: &mut Database) -> Markup {
    html!(
        div id="top_bar" {
            @if let Some(user) = &database.current_user {
                span { "hello, " (user.username) "!" }

            } @else {
                span { "you are not logged in!" }
            }
        }
    )
}
