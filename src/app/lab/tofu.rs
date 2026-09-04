mod blocks;
mod chars;

use crate::app::AppContext;
use crate::{CodePoint, components::breadcrumbs};

#[::topcoat::router::query_params(error = bad_request)]
struct RootQueryParams {
    q: Option<String>,
}

#[::topcoat::router::page]
async fn root(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let query = ::topcoat::router::query_params::<RootQueryParams>(cx)?;
    let app_context = ::topcoat::context::app_context::<AppContext>(cx);
    let names_list = &app_context.names_list;
    let readings = &app_context.readings;
    match &query.q {
        Some(s) if s.chars().count() == 1 => match s.chars().next() {
            None => {}
            Some(c) => {
                let code_point = CodePoint::from_char(c);
                if names_list.get(&code_point).is_some() || readings.get(&code_point).is_some() {
                    return ::topcoat::Result::<::topcoat::view::View, ::topcoat::Error>::Err(
                        ::topcoat::router::error::redirect(&format!(
                            "/lab/tofu/chars/{}",
                            code_point.to_string_without_u_plus()
                        ))
                        .into(),
                    );
                } else {
                    return ::topcoat::Result::<::topcoat::view::View, ::topcoat::Error>::Err(
                        ::topcoat::router::error::not_found().into(),
                    );
                }
            }
        },
        Some(_) | None => {}
    }
    let version = env!("CARGO_PKG_VERSION");

    ::topcoat::view::view! {
        <div class="page home-page">
            breadcrumbs(
                items: vec![("/lab/tofu", Some(::topcoat::asset::asset!("./tofu.svg")), "tofu")]
            )
            <h1>"tofu"</h1>
            <form method="get" action="/lab/tofu">
                <input name="q" placeholder="A, あ, 📛, etc." type="text" />
                <button type="submit">"Search"</button>
            </form>
            <h2>"Menu"</h2>
            <ul>
                <li><a href="/lab/tofu/blocks">"Blocks"</a></li>
                <li><a href="/lab/tofu/chars">"Characters"</a></li>
            </ul>
            <h2>"Data Sources"</h2>
            <ul>
                <li>
                    <a
                        href="https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt"
                    </a>
                </li>
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
                        href="https://www.unicode.org/Public/UCD/latest/ucd/Unihan.zip"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "https://www.unicode.org/Public/UCD/latest/ucd/Unihan.zip"
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
