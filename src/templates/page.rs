use axum::extract::Path;
use maud::{html, Markup, DOCTYPE};

pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (header(title))
        (nav())
        h1 { (title) }
        (body)
        (footer())
    }
}

pub async fn root() -> Markup {
    page("Cool Site", home())
}

pub async fn hello(name: Option<Path<String>>) -> Markup {
    let name = name.map(|Path(n)| n).unwrap_or_else(|| "Boss".to_string());
    page("Cool Site", greeting(&name))
}

pub fn greeting(name: &str) -> Markup {
    html! {
        h2 {"Hello, " (name) "!"}
        div {"Center a div? No way"}
    }
}

pub fn home() -> Markup {
    html! {
        h2 { "I have no idea what I'm doing"}
        div {
            "Hello,"
            br;
            "This is dog..."
            br;
            "Cool site eh?"
        }
    }
}

pub fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (title) }
    }
}

pub fn footer() -> Markup {
    html! {
        footer {
            p { "Written in Rust"}
        }
    }
}

pub fn nav() -> Markup {
    html! {
        a href="/" { "Home" }
        span { " : " }
        a href="/hello" { "Hello" }
        span { " : " }
        a href="/hello/reese" { "Greeting"}
    }
}
