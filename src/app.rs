mod lab;

use crate::CodePoint;

struct AppContext {
    pub blocks: Vec<super::Block>,
    pub names_list: std::collections::HashMap<CodePoint, super::CharEntry>,
}

pub fn router(
    blocks: Vec<super::Block>,
    names_list: std::collections::HashMap<CodePoint, super::CharEntry>,
) -> ::topcoat::router::Router {
    use topcoat::asset::RouterBuilderAssetExt as _;
    use topcoat::cookie::RouterBuilderCookieExt as _;
    use topcoat::session::RouterBuilderSessionExt as _;

    ::topcoat::router::module_router!()
        .app_context(AppContext { blocks, names_list })
        .cookies()
        .sessions(::topcoat::session::SessionConfig::default())
        .app_context(::topcoat::cookie::Key::generate())
        .assets(::topcoat::asset::AssetBundle::load().unwrap())
        .build()
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
                <link
                    href=(::topcoat::asset::asset!("./favicon.svg"))
                    rel="icon"
                    type="image/svg+xml"
                />
                <link href=(::topcoat::asset::asset!("./index.css")) rel="stylesheet" />
                <title>"tofu"</title>
                ::topcoat::dev::script()
            </head>
            <body>(content)</body>
        </html>
    }
}

#[::topcoat::router::layer]
async fn log_layer(
    cx: &::topcoat::context::Cx,
    body: ::topcoat::router::Body,
    next: ::topcoat::router::Next<'_>,
) -> ::topcoat::Result<::topcoat::router::response::Response> {
    let start = std::time::Instant::now();
    let response = next.run(cx, body).await?;
    let status = response.status();
    println!("-> {} ({:?})", status, start.elapsed());
    Ok(response)
}
