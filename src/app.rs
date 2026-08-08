mod example;
mod hello;

pub fn router() -> ::topcoat::router::Router {
    ::topcoat::router::module_router!().build()
}

#[::topcoat::router::page]
async fn root() -> ::topcoat::Result {
    ::topcoat::view::view! {
        <p>"OK"</p>
    }
}

#[::topcoat::router::layout]
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

#[::topcoat::router::layer]
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
