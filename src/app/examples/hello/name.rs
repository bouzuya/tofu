#[::topcoat::router::path_param(error = not_found)]
struct Name(String);

#[::topcoat::router::page]
async fn hello_page(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let name = ::topcoat::router::path_param::<Name>(cx)?;
    ::topcoat::view::view! {
        (::topcoat::router::StatusCode::OK)
        hello(name: name)
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
