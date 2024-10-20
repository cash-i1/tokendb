use maud::html;
use maud::Markup;

pub fn head() -> Markup {
    html!(
        head {
            link rel="stylesheet" href="/assets/styles.css" {}
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
    )
}
