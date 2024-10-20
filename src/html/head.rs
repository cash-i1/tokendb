use maud::html;
use maud::Markup;

pub fn head() -> Markup {
    html!(
        head {
            meta name="viewport" content="width=device-width, initial-scale=1.0" {}
            link 
                rel="stylesheet"
                href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css" {}

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
