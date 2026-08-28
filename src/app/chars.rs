mod code_point;

use crate::components::{breadcrumbs, character};

#[::topcoat::router::query_params(error = bad_request)]
struct GetCharsParams {
    start: Option<String>,
}

#[::topcoat::router::page]
async fn get_chars(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let app_context = ::topcoat::context::app_context::<crate::app::AppContext>(cx);
    let params = ::topcoat::router::query_params::<GetCharsParams>(cx)?;
    let start = u32::from_str_radix(&params.start.clone().unwrap_or("0".to_string()), 16)
        .unwrap_or(0)
        .clamp(0x0000, 0x10FFFF);
    let limit = 20_u32;
    ::topcoat::view::view! {
        breadcrumbs(items: vec![("/", "Home"), ("/chars", "Characters")])
        <h1>"Characters"</h1>
        <ul>
            for (code_point, char_entry) in (start..=0x10FFFF)
                .filter_map(|code_point_as_u32| {
                    crate::CodePoint::from_u32(code_point_as_u32)
                        .and_then(|code_point| {
                            app_context
                                .names_list
                                .get(&code_point)
                                .map(|name| (code_point, name))
                        })
                })
                .take(usize::try_from(limit).expect("limit is too large")) {
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
                        (char_entry.name.clone())
                    </a>
                </li>
            }
        </ul>
        <nav>
            <ul>
                if start == 0 {
                    <li>"Previous"</li>
                } else {
                    <li>
                        <a
                            href=(format!(
                                "/chars?start={:04X}", start.saturating_sub(limit)
                            ))
                        >
                            "Previous"
                        </a>
                    </li>
                }
                if start == 0x10FFFF {
                    <li>"Next"</li>
                } else {
                    <li>
                        <a
                            href=(format!(
                                "/chars?start={:04X}", start.saturating_add(limit)
                            ))
                        >
                            "Next"
                        </a>
                    </li>
                }
            </ul>
        </nav>
    }
}
