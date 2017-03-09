use maud::{Markup, Render};

pub struct Meta(pub &'static str, pub &'static str);

impl Render for Meta {
    fn render(&self) -> Markup {
        html! {
            meta name=(self.0) content=(self.1) /
        }
    }
}

pub struct CSS(pub &'static str);

impl Render for CSS {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0) /
        }
    }
}

pub struct JS(pub &'static str, pub bool);

impl Render for JS {
    fn render(&self) -> Markup {
        html! {
            script defer?[self.1] type="text/javascript" src=(self.0) {}
        }
    }
}
