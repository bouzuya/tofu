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
        badge(label: "New", tone: "success")
        <div>count(items: vec!["a", "b", "c"])</div>
        <div>count(items: vec![1, 2, 3, 4, 5])</div>
        current_path()
        trampoline1(n: 10)
        attributes()
        my_section(
            attrs: ::topcoat::view::attributes! { class="my-section" },
            <p>"My section"</p>
        )
        class_macro()
    }
}

#[::topcoat::view::component]
async fn attributes() -> ::topcoat::Result {
    let id = "button-id";
    let attrs = ::topcoat::view::attributes! {
        class="button"
        id=(id)
        :data-bound=$(id.to_owned())
        @input="(e) => console.log(e)"
        if id == "submit" {
            type="submit"
        } else {
            type="button"
        }
        for (name, value) in [("foo", "1"), ("bar", "2"), ("baz", "3")] {
            (name)=(value)
        }
        match id {
            "submit" => aria-label="Submit",
            _ => aria-label="Button",
        }
    };
    ::topcoat::view::view! { <button (attrs)>"button"</button> }
}

// #[into] v: T ... v: impl Into<T>
#[::topcoat::view::component]
async fn badge(#[into] label: String, tone: &str) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <span class=(format!("badge badge-{}", tone))>(label)</span>
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
async fn class_macro() -> ::topcoat::Result {
    let variant = Some("primary");
    let classes = vec!["small", "large"];
    let enabled = true;
    ::topcoat::view::view! {
        <button
            class=(::topcoat::view::class! {
                "button",
                variant,
                classes,
                "enabled" if enabled else "disabled",
            })
        >
            "button"
        </button>
    }
}

#[::topcoat::view::component]
async fn count<T: Send + Sync>(items: Vec<T>) -> ::topcoat::Result {
    ::topcoat::view::view! { <span>(items.len())</span> }
}

#[::topcoat::view::component]
async fn current_path(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    ::topcoat::view::view! { <span>(::topcoat::router::uri(cx).path())</span> }
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

#[::topcoat::router::path_param(error = not_found)]
struct Name(String);

#[::topcoat::router::page("/hello/{name}")]
async fn hello_page(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let name = ::topcoat::router::path_param::<Name>(cx)?;
    ::topcoat::view::view! {
        (::topcoat::router::StatusCode::OK)
        hello(name: name)
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
async fn my_section(
    attrs: ::topcoat::view::Attributes,
    child: ::topcoat::view::View,
) -> ::topcoat::Result {
    ::topcoat::view::view! { <section (attrs)>(child)</section> }
}

#[::topcoat::view::component]
async fn option_attr() -> ::topcoat::Result {
    ::topcoat::view::view! { <p class=(Some("active"))>"Hello"</p> }
}

#[::topcoat::router::layout("/")]
async fn root_layout(slot: ::topcoat::Result) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <title>"tofu"</title>
                ::topcoat::dev::script()
            </head>
            <body>(slot?)</body>
        </html>
    }
}

#[::topcoat::router::layer("/")]
async fn log_layer(
    cx: &mut ::topcoat::context::CxBuilder,
    body: ::topcoat::router::Body,
    next: ::topcoat::router::Next<'_>,
) -> ::topcoat::Result<::topcoat::router::Response> {
    let start = std::time::Instant::now();
    let response = next.run(cx, body).await?;
    let status = response.status();
    println!("-> {} ({:?})", status, start.elapsed());
    Ok(response)
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

#[::topcoat::view::component(boxed)]
async fn trampoline1(n: i32) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <div>
            "trampoline1"
            <p>(n)</p>
            if n > 0 {
                trampoline2(n: n - 1)
            } else {
                <p>"Done"</p>
            }
        </div>
    }
}

#[::topcoat::view::component(boxed)]
async fn trampoline2(n: i32) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <div>
            "trampoline2"
            <p>(n)</p>
            if n > 0 {
                trampoline1(n: n - 1)
            } else {
                <p>"Done"</p>
            }
        </div>
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
