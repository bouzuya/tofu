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
    let start_code =
        u32::from_str_radix(parsed[0], 16).map_err(|_| ::topcoat::router::error::not_found())?;
    let end_code =
        u32::from_str_radix(parsed[1], 16).map_err(|_| ::topcoat::router::error::not_found())?;
    let app_context = ::topcoat::context::app_context::<crate::app::AppContext>(cx);
    let block = app_context
        .blocks
        .iter()
        .find(|block| block.code_range == (start_code..=end_code))
        .ok_or_else(|| ::topcoat::router::error::not_found())?;
    ::topcoat::view::view! {
        <h1>
            (format!("U+{:04X}..U+{:04X}", start_code, end_code))
            " "
            (format!("{}", block.block_name))
        </h1>
        <ul>
            for code_point in block.code_range.clone() {
                match app_context.names_list.get(&code_point) {
                    None => {
                        <li>
                            (format!("U+{:04X}", code_point))
                            " "
                            "<unknown>"
                        </li>
                    }
                    Some(entry) => {
                        <li>
                            <a href=(format!("/chars/{:04X}", code_point))>
                                (format!("U+{:04X}", code_point))
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
