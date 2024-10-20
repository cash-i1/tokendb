use crate::database::Database;
use maud::html;
use maud::Markup;

pub fn root(database: &mut Database) -> Markup {
    html!(
        head {
            script {(maud::PreEscaped(
                r#"
                async function i_have_token() {
                    let token = localStorage.getItem("token");
                    let resp = await fetch(`/api/i_have_token/${token}`);
                    console.log(await resp.text());
                }
                i_have_token();
                "#))
            }
        }
        (super::top_bar(database))
        h1 { "tokendb" }
        a href="/transfer" { "transfer" }
    )
}
