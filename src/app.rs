mod api;
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
    let content = match slot {
        Err(error)
            if error
                .downcast_ref::<::topcoat::router::error::NotFoundError>()
                .is_some() =>
        {
            ::topcoat::view::view! {
                (::topcoat::router::StatusCode::NOT_FOUND)
                "Not Found"
            }
        }
        content => content,
    }?;
    ::topcoat::view::view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <title>"tofu"</title>
                ::topcoat::dev::script()
            </head>
            <body>(content)</body>
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
