use topcoat::router::error::RouterErrorExt;

use crate::app::AppContext;
use crate::components::{breadcrumbs, character};

::topcoat::router::path_param!(code_point: crate::code_point::CodePoint);

#[::topcoat::router::page]
async fn get_code_point<'a>(cx: &'a ::topcoat::context::Cx) -> ::topcoat::Result {
    let code_point =
        ::topcoat::router::path_param::<CodePoint>(cx).ok_or_bad_request("invalid code_point")?;
    let char_ = code_point
        .to_char()
        .ok_or_else(::topcoat::router::error::not_found)?;
    let app_context = ::topcoat::context::app_context::<AppContext>(cx);
    let block = app_context
        .blocks
        .iter()
        .find(|block| block.code_range.contains(code_point))
        .ok_or_else(::topcoat::router::error::not_found)?;
    let char_url = format!("/lab/tofu/chars/{}", code_point.to_string_without_u_plus());
    let char_text = format!(
        "{} {}",
        code_point.to_string_with_u_plus(),
        app_context
            .names_list
            .get(code_point)
            .map(|entry| &entry.name)
            .unwrap_or(&"<unknown>".to_string())
    );
    ::topcoat::view::view! {
        <div class="page char-detail-page">
            breadcrumbs(
                items: vec![
                ("/lab/tofu", None, "tofu"),
                ("/lab/tofu/chars", None, "Characters"),
                (& char_url, None, &char_text),
            ]
            )
            <h1>(&char_text)</h1>
            <div class="character-container">character(c: char_, thumbnail: false)</div>
            <div>
                <div class="label">"aliases:"</div>
                <div class="value">
                    match app_context
                        .names_list
                        .get(code_point)
                        .map(|entry| &entry.aliases) {
                        Some(aliases) => {
                            if !aliases.is_empty() {
                                <ul>
                                    for alias in aliases {
                                        <li>(alias)</li>
                                    }
                                </ul>
                            } else {
                                "(none)"
                            }
                        }
                        None => {
                            "(none)"
                        }
                    }
                </div>
            </div>
            <div>
                <div class="label">"block: "</div>
                <div class="value">
                    <a
                        href=(format!(
                    "/lab/tofu/blocks/{}..{}", block.code_range.start()
                    .to_string_without_u_plus(), block.code_range.end()
                    .to_string_without_u_plus()
                ))
                    >
                        (block.code_range.start().to_string_with_u_plus())
                        ".."
                        (block.code_range.end().to_string_with_u_plus())
                        " "
                        (format!("{}", block.block_name))
                    </a>
                </div>
            </div>
            <div>
                <div class="label">"comments:"</div>
                <div class="value">
                    match app_context
                        .names_list
                        .get(code_point)
                        .map(|entry| &entry.comments) {
                        Some(comments) => {
                            if !comments.is_empty() {
                                <ul>
                                    for comment in comments {
                                        <li>(comment)</li>
                                    }
                                </ul>
                            } else {
                                "(none)"
                            }
                        }
                        None => {
                            "(none)"
                        }
                    }
                </div>
            </div>
            <div>
                <div class="label">"cross_refs:"</div>
                <div class="value">
                    match app_context
                        .names_list
                        .get(code_point)
                        .map(|entry| &entry.cross_refs) {
                        Some(cross_refs) => {
                            if !cross_refs.is_empty() {
                                <ul>
                                    for (name, code_point) in cross_refs {
                                        <li>
                                            <a
                                                href=(format!(
                                            "/lab/tofu/chars/{}", code_point.to_string_without_u_plus()
                                        ))
                                            >
                                                character(
                                                    c: code_point.to_char().unwrap_or(' '),
                                                    thumbnail: true
                                                )
                                                " "
                                                (code_point.to_string_with_u_plus())
                                                " "
                                                (name.to_ascii_uppercase())
                                            </a>
                                        </li>
                                    }
                                </ul>
                            } else {
                                "(none)"
                            }
                        }
                        None => {
                            "(none)"
                        }
                    }
                </div>
            </div>
        </div>
    }
}
