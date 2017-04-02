use web::common::page;
use maud::Markup;

#[get("/")]
pub fn web_root() -> Markup {
    let content = html!{
        div.ac-layout-transparent.mdl-layout.mdl-js-layout {
            header.mdl-layout__header {
                div.mdl-layout__header-row {
                    span.mdl-layout-title "Auracite"
                    div.mdl-layout-spacer {}
                    nav.mdl-navigation {
                        a.mdl-navigation__link href="/lodestone" "Lodestone"
                    }
                }
            }
            div.mdl-layout__drawer {
                span.mdl-layout-title "Auracite"
                nav.mdl-navigation {
                    a.mdl-navigation__link href="/lodestone" "Lodestone"
                }
            }
            main.mdl-layout__content {
                div.page-content {
                    (hero())
                }
            }
        }
    };

    let styles = html!{
        style type="text/css" { (include_str!("../res/hero.css")) }
    };

    page(&content, Some(styles))
}

fn hero() -> Markup {
    html!{
        div.hero {
            img src="http://placehold.it/250"
            h1 "Auracite"
        }
    }
}
