use crate::app::AppContext;

#[::topcoat::router::path_param(error = bad_request)]
struct CodePoint(String);

#[::topcoat::router::page]
async fn get_code_point(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let code_point_value = ::topcoat::router::path_param::<CodePoint>(cx)?;
    let code_point_value = code_point_value
        .chars()
        .all(|c| matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F'))
        .then(|| code_point_value)
        .and_then(|it| u32::from_str_radix(&it, 16).ok())
        .and_then(|it| matches!(it, 0x0..=0x10FFFF).then(|| it))
        .and_then(std::char::from_u32)
        .ok_or_else(|| ::topcoat::router::error::not_found())?;
    let app_context = ::topcoat::context::app_context::<AppContext>(cx);
    let block = app_context
        .blocks
        .iter()
        .find(|block| block.code_range.contains(&(code_point_value as u32)))
        .ok_or_else(|| ::topcoat::router::error::not_found())?;
    ::topcoat::view::view! {
        <h1>"Code Point"</h1>
        <p>
            "code_point: "
            (format!("{:?}", code_point_value))
        </p>
        <p>
            "block: "
            (format!("{}", block.block_name))
        </p>
    }
}
