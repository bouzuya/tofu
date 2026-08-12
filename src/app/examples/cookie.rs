#[::topcoat::router::page]
async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    use topcoat::cookie::Cookies as _;

    let jar = my_signed_cookies(cx);

    jar.add(
        ::topcoat::cookie::Cookie::build(("foo", "bar"))
            .path("/")
            .build(),
    );
    jar.add(::topcoat::cookie::cookie! { "bar" = "baz"; Path = "/"; HttpOnly; SameSite = Lax });

    ::topcoat::view::view! {
        <h1>"Cookie"</h1>
        <p>"Check the cookie in the browser devtools."</p>
    }
}

fn my_signed_cookies(cx: &::topcoat::context::Cx) -> impl ::topcoat::cookie::Cookies {
    use topcoat::cookie::Cookies as _;

    ::topcoat::cookie::signed_cookies(cx)
        .default_secure(false)
        .default_http_only(true)
        .default_same_site(::topcoat::cookie::SameSite::Lax)
        .default_path("/")
}
