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

pub async fn family_body() -> Markup {
    page("Add Family", family_form().await)
}

pub async fn family_form() -> Markup {
    html! {
        h2 { "Create a new family (not like that)" }
        form action="submit" method="post" {
            label for="name" { "Family name (or nickname): " }
            input type="text" name="name" id="name" required {};
            input type="submit" value="Submit" {};
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
        a href="/add_family" { "Add Family" }
        span { " : " }
        a href="/hello/reese" { "Greeting"}
    }
}
