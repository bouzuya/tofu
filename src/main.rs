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
            </body>
        </html>
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
