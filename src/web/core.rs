use maud::Markup;

#[error(404)]
pub fn not_found() -> Markup {
    html!{
        h1 "404"
    }
}
