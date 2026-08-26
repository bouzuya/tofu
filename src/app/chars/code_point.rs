use crate::app::AppContext;
use crate::components::character;

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
    ::topcoat::view::view! {
        <nav class="breadcrumb-list">
            <ol>
                <li><a href="/">"Home"</a></li>
                <li><a href="/chars">"Chars"</a></li>
                <li>
                    <a
                        href=(format!(
                            "/chars/{}", code_point.to_string_without_u_plus()
                        ))
                    >
                        (code_point.to_string_with_u_plus())
                    </a>
                </li>
            </ol>
        </nav>
        <h1>
            (code_point.to_string_with_u_plus())
            " "
            (format!(
                "{}", app_context.names_list.get(& code_point).map(| entry | & entry
                .name).unwrap_or(& "<unknown>".to_string())
            ))
        </h1>
        character(c: char_, thumbnail: false)
        <div>
            "aliases:"
            <ul>
                for alias in app_context
                    .names_list
                    .get(&code_point)
                    .map(|entry| &entry.aliases)
                    .unwrap_or(&Vec::new()) {
                    <li>(alias)</li>
                }
            </ul>
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
            <ul>
                for comment in app_context
                    .names_list
                    .get(&code_point)
                    .map(|entry| &entry.comments)
                    .unwrap_or(&Vec::new()) {
                    <li>(comment)</li>
                }
            </ul>
        </div>
        <div>
            "cross_refs:"
            <ul>
                for (name, code_point) in app_context
                    .names_list
                    .get(&code_point)
                    .map(|entry| &entry.cross_refs)
                    .unwrap_or(&Vec::new()) {
                    <li>
                        <a
                            href=(format!(
                                "/chars/{}", code_point.to_string_without_u_plus()
                            ))
                        >
                            (code_point.to_string_with_u_plus())
                            " "
                            (name.to_ascii_uppercase())
                        </a>
                    </li>
                }
            </ul>
        </div>
    }
}
