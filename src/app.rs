use crate::{CodePoint, components::breadcrumbs};

mod blocks;
mod chars;

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

#[::topcoat::router::query_params(error = bad_request)]
struct RootQueryParams {
    q: Option<String>,
}

#[::topcoat::router::page]
async fn root(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let query = ::topcoat::router::query_params::<RootQueryParams>(cx)?;
    let names_list = &::topcoat::context::app_context::<AppContext>(cx).names_list;
    match &query.q {
        Some(s) if s.chars().count() == 1 => match s.chars().next() {
            None => {}
            Some(c) => {
                let code_point = CodePoint::from_char(c);
                match names_list.get(&code_point) {
                    Some(_entry) => {
                        return ::topcoat::Result::<::topcoat::view::View, ::topcoat::Error>::Err(
                            ::topcoat::router::error::redirect(&format!(
                                "/chars/{}",
                                code_point.to_string_without_u_plus()
                            ))
                            .into(),
                        );
                    }
                    None => {
                        return ::topcoat::Result::<::topcoat::view::View, ::topcoat::Error>::Err(
                            ::topcoat::router::error::not_found().into(),
                        );
                    }
                }
            }
        },
        Some(_) | None => {}
    }
    let version = env!("CARGO_PKG_VERSION");

    ::topcoat::view::view! {
        <div class="page home-page">
            breadcrumbs(
                items: vec![("/", Some(::topcoat::asset::asset!("./tofu.svg")), "tofu")]
            )
            <h1>"tofu"</h1>
            <form method="get" action="/">
                <input name="q" placeholder="A, あ, 📛, etc." type="text" />
                <button type="submit">"Search"</button>
            </form>
            <h2>"Menu"</h2>
            <ul>
                <li><a href="/blocks">"Blocks"</a></li>
                <li><a href="/chars">"Characters"</a></li>
            </ul>
            <h2>"Data Sources"</h2>
            <ul>
                <li>
                    <a
                        href="https://www.unicode.org/Public/UCD/latest/ucd/NamesList.txt"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "https://www.unicode.org/Public/UCD/latest/ucd/NamesList.txt"
                    </a>
                </li>
                <li>
                    <a
                        href="https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt"
                    </a>
                </li>
            </ul>
            <h2>"About"</h2>
            <div class="about">
                <div class="about-icon">
                    <img
                        alt=""
                        height="64"
                        src=(::topcoat::asset::asset!("./tofu.svg"))
                        width="64"
                    />
                </div>
                <div class="about-text">
                    <div class="brand">"tofu"</div>
                    <div class="version">
                        "v"
                        (version)
                    </div>
                    <a
                        href="https://github.com/bouzuya/tofu"
                        rel="noopener noreferrer"
                        target="_blank"
                    >
                        "Source Code"
                    </a>
                </div>
            </div>
        </div>
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
