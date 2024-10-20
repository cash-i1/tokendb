use crate::database::Database;
use maud::html;
use maud::Markup;

pub fn root(database: &mut Database) -> Markup {
    html!(
        (super::head())
        (super::top_bar(database))
        h1 { "tokendb" }
        a href="/transfer" { "transfer" }
        br {}
        a href="/balance" { "balance" }
        br {}
        a href="/account" { "account" }
    )
}
