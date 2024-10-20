use crate::user::User;
use maud::html;
use maud::Markup;

pub fn root(user: &mut Option<User>) -> Markup {
    html!(
        (super::top_bar(user))
        h1 { "tokendb" }
        a href="/transfer" { "transfer" }
    )
}
