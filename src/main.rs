#[::tokio::main]
async fn main() {
    ::topcoat::start(
        ::topcoat::router::RouterBuilderDiscoverExt::discover(::topcoat::router::Router::builder())
            .build(),
    )
    .await
    .unwrap();
}

#[::topcoat::router::page("/")]
async fn home() -> ::topcoat::Result {
    ::topcoat::view::view! {
        (::topcoat::router::StatusCode::OK)
        let user = User {
            name: "bouzuya".to_string(),
        };
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <title>"tofu"</title>
                ::topcoat::dev::script()
            </head>
            <body>
                <h1>"tofu"</h1>
                hello(name: "World")
                sidebar()
                custom_element()
                user_name(user: &user)
                if_expr(is_open: true)
                for_expr()
                match_expr(fruit: Fruit::Banana)
                let_stmt()
                child_prop(label: "Label", <span>"Child"</span>)
                bool_attr(disabled: true)
                option_attr()
                (CustomNodeViewParts("Custom view parts".to_string()))
            </body>
        </html>
    }
}

#[::topcoat::view::component]
async fn bool_attr(disabled: bool) -> ::topcoat::Result {
    ::topcoat::view::view! { <button disabled=(disabled)>"button"</button> }
}

#[::topcoat::view::component]
async fn child_prop(label: &str, child: ::topcoat::view::View) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <div>
            <div class="label">(label)</div>
            <div class="value">(child)</div>
        </div>
    }
}

#[::topcoat::view::component]
async fn custom_element() -> ::topcoat::Result {
    ::topcoat::view::view! { <my-element>"Hello, World!"</my-element> }
}

struct CustomNodeViewParts(String);

impl ::topcoat::view::NodeViewParts for CustomNodeViewParts {
    fn into_view_parts(
        self,
        _cx: &topcoat::context::Cx,
        parts: &mut topcoat::view::PartsWriter<'_>,
    ) {
        parts.push_str(self.0);
    }
}

#[::topcoat::view::component]
async fn dynamic_element() -> ::topcoat::Result {
    let tag = "a";
    let attr = "href";
    ::topcoat::view::view! { <(tag) (attr)="https://bouzuya.net">"bouzuya.net"</(tag)> }
}

#[::topcoat::view::component]
async fn for_expr() -> ::topcoat::Result {
    ::topcoat::view::view! {
        <ul
            for (name, value) in [("foo", 1), ("bar", 2), ("baz", 3)] {
                (name)=(value)
            }
        >
            for i in 1..=10 {
                <li>
                    "Item "
                    (i)
                </li>
            }
        </ul>
    }
}

#[::topcoat::view::component]
async fn hello(name: &str) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <h1>
            "Hello, "
            (name)
            "!"
        </h1>
    }
}

#[::topcoat::view::component]
async fn if_expr(is_open: bool) -> ::topcoat::Result {
    ::topcoat::view::view! {
        if is_open {
            <p
                if is_open {
                    class="open"
                }
            >
                "Open"
            </p>
        } else {
            <p>"Closed"</p>
        }
    }
}

#[derive(Clone, Copy)]
enum Fruit {
    Apple,
    Banana,
    Cherry,
}

#[::topcoat::view::component]
async fn let_stmt() -> ::topcoat::Result {
    ::topcoat::view::view! {
        <div>
            let fruit = Fruit::Apple;
            <p>
                "Fruit: "
                match fruit {
                    Fruit::Apple => "Apple",
                    Fruit::Banana => "Banana",
                    Fruit::Cherry => "Cherry",
                }
            </p>
        </div>
    }
}

#[::topcoat::view::component]
async fn match_expr(fruit: Fruit) -> ::topcoat::Result {
    ::topcoat::view::view! {
        match fruit {
            Fruit::Apple => <p>"Red"</p>,
            Fruit::Banana => <p>"Yellow"</p>,
            Fruit::Cherry => <p>"Dark red"</p>,
        }
    }
}

#[::topcoat::view::component]
async fn option_attr() -> ::topcoat::Result {
    ::topcoat::view::view! { <p class=(Some("active"))>"Hello"</p> }
}

#[::topcoat::view::component]
async fn sidebar() -> ::topcoat::Result {
    let url = "https://bouzuya.net/";
    ::topcoat::view::view! {
        <ul>
            <li><a href="/">"Home"</a></li>
            <li><a href=(url)>"bouzuya.net"</a></li>
            for i in 1..=10 {
                <li>
                    <a href=(format!("/hello/{}", i))>
                        if i % 3 == 0 && i % 5 == 0 {
                            "FizzBuzz"
                        } else if i % 3 == 0 {
                            "Fizz"
                        } else if i % 5 == 0 {
                            "Buzz"
                        } else {
                            (i)
                        }
                    </a>
                </li>
            }
        </ul>
    }
}

struct User {
    name: String,
}

#[::topcoat::view::component]
async fn user_name(user: &User) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <p>
            "User name: "
            (&user.name)
        </p>
    }
}
