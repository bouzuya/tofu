use crate::app::AppContext;
use crate::components::{breadcrumbs, character};

#[::topcoat::router::path_param(error = bad_request)]
struct CodePoint(pub crate::code_point::CodePoint);

#[::topcoat::router::page]
async fn get_code_point(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let code_point = ::topcoat::router::path_param::<CodePoint>(cx)?;
    let char_ = code_point
        .to_char()
        .ok_or_else(|| ::topcoat::router::error::not_found())?;
    let app_context = ::topcoat::context::app_context::<AppContext>(cx);
    let block = app_context
        .blocks
        .iter()
        .find(|block| block.code_range.contains(&code_point))
        .ok_or_else(|| ::topcoat::router::error::not_found())?;
    let char_url = format!("/chars/{}", code_point.to_string_without_u_plus());
    let char_text = format!(
        "{} {}",
        code_point.to_string_with_u_plus(),
        app_context
            .names_list
            .get(&code_point)
            .map(|entry| &entry.name)
            .unwrap_or(&"<unknown>".to_string())
    );
    ::topcoat::view::view! {
        <div class="page">
            breadcrumbs(
                items: vec![
                ("/", None, "tofu"),
                ("/chars", None, "Characters"),
                (& char_url, None, &char_text),
            ]
            )
            <h1>(char_text)</h1>
            character(c: char_, thumbnail: false)
            <div>
                "aliases:"
                match app_context
                    .names_list
                    .get(&code_point)
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
            <p>
                "block: "
                <a
                    href=(format!(
                    "/blocks/{}..{}", block.code_range.start()
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
            </p>
            <div>
                "comments:"
                match app_context
                    .names_list
                    .get(&code_point)
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
            <div>
                "cross_refs:"
                match app_context
                    .names_list
                    .get(&code_point)
                    .map(|entry| &entry.cross_refs) {
                    Some(cross_refs) => {
                        if !cross_refs.is_empty() {
                            <ul>
                                for (name, code_point) in cross_refs {
                                    <li>
                                        <a
                                            href=(format!(
                                            "/chars/{}", code_point.to_string_without_u_plus()
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
    }
}
