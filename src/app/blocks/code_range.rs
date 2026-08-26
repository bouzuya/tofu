use crate::{CodePoint, components::character};

#[::topcoat::router::path_param(error = not_found)]
struct CodeRange(String);

#[::topcoat::router::page]
async fn get_block(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let code_range_value = ::topcoat::router::path_param::<CodeRange>(cx)?;
    let parsed = code_range_value
        .split_terminator("..")
        .collect::<Vec<&str>>();
    if parsed.len() != 2 {
        return Err(::topcoat::router::error::not_found().into());
    }
    let start_code = CodePoint::from_str_without_u_plus(parsed[0])
        .ok_or_else(|| ::topcoat::router::error::not_found())?;
    let end_code = CodePoint::from_str_without_u_plus(parsed[1])
        .ok_or_else(|| ::topcoat::router::error::not_found())?;
    let app_context = ::topcoat::context::app_context::<crate::app::AppContext>(cx);
    let block = app_context
        .blocks
        .iter()
        .find(|block| block.code_range == (start_code..=end_code))
        .ok_or_else(|| ::topcoat::router::error::not_found())?;
    ::topcoat::view::view! {
        <nav class="breadcrumb-list">
            <ol>
                <li><a href="/">"Home"</a></li>
                <li><a href="/blocks">"Blocks"</a></li>
                <li>
                    <a
                        href=(format!(
                            "/blocks/{}..{}", start_code.to_string_without_u_plus(),
                            end_code.to_string_without_u_plus()
                        ))
                    >
                        (start_code.to_string_with_u_plus())
                        ".."
                        (end_code.to_string_with_u_plus())
                        " "
                        (block.block_name.clone())
                    </a>
                </li>
            </ol>
        </nav>
        <h1>
            (start_code.to_string_with_u_plus())
            ".."
            (end_code.to_string_with_u_plus())
            " "
            (block.block_name.clone())
        </h1>
        <ul>
            for code_point_as_u32 in block
                .code_range
                .start()
                .to_u32()..=block.code_range.end().to_u32() {
                let code_point = match CodePoint::from_u32(code_point_as_u32) {
                    None => continue,
                    Some(code_point) => code_point,
                };
                match app_context.names_list.get(&code_point) {
                    None => {
                        <li>
                            (code_point.to_string_with_u_plus())
                            " "
                            "<unknown>"
                        </li>
                    }
                    Some(entry) => {
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
                                (entry.name.to_ascii_uppercase())
                            </a>
                        </li>
                    }
                }
            }
        </ul>
    }
}
