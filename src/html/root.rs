use crate::database::Database;
use maud::html;
use maud::Markup;

pub fn root(database: &mut Database) -> Markup {
    html!(
        (super::top_bar(database))
        h1 { "tokendb" }
        a href="/transfer" { "transfer" }
    )
}
