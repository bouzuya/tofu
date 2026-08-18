use std::sync::atomic::AtomicU16;

mod api;
mod context;
mod example;
mod examples;
mod hello;
mod u;

struct AppContext {
    pub blocks: Vec<super::Block>,
    pub count: AtomicU16,
    pub names_list: std::collections::HashMap<u32, String>,
}

pub fn router(
    blocks: Vec<super::Block>,
    names_list: std::collections::HashMap<u32, String>,
) -> ::topcoat::router::Router {
    use topcoat::cookie::RouterBuilderCookieExt as _;
    use topcoat::session::RouterBuilderSessionExt as _;

    ::topcoat::router::module_router!()
        .app_context(AppContext {
            blocks,
            count: AtomicU16::new(0),
            names_list,
        })
        .cookies()
        .sessions(::topcoat::session::SessionConfig::default())
        .app_context(::topcoat::cookie::Key::generate())
        .build()
}

#[::topcoat::router::page]
async fn root() -> ::topcoat::Result {
    ::topcoat::view::view! { <p>"OK"</p> }
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
