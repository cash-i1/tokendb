use crate::user::User;
use maud::html;
use maud::Markup;

pub fn transfer(user: &mut Option<User>) -> Markup {
    html!(
        (super::top_bar(user))
        h1 { "transfer" }
        select {
            @for user in ["1", "2"] {
                option { (user) }
            }
        }
        span { "->" }
        select {
            @for user in ["1", "2"] {
                option { (user) }
            }
        }
        button disabled[user.is_none()] { "transfer" }
    )
}
