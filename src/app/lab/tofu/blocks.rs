use crate::components::breadcrumbs;

pub mod code_range;

#[::topcoat::router::page]
pub async fn page(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let app_context = ::topcoat::context::app_context::<crate::app::AppContext>(cx);
    Ok(::topcoat::view::view! {
        <div class="page block-list-page">
            breadcrumbs(
                items: vec![
                    ("/lab/tofu", Some(::topcoat::asset::asset!("../tofu.svg")), "tofu"),
                    ("/lab/tofu/blocks", None, "Blocks"),
                ]
            )
            <h1>"Blocks"</h1>
            <ul>
                for block in &app_context.blocks {
                    <li>
                        <a
                            href=(::topcoat::router::href!(
                                crate::app::lab::tofu::blocks::code_range::page,
                                crate::app::lab::tofu::blocks::code_range::CodeRange(
                                    format!(
                                        "{}..{}",
                                        block.code_range.start().to_string_without_u_plus(),
                                        block.code_range.end().to_string_without_u_plus(),
                                    ),
                                ),
                            ))
                        >
                            (block.code_range.start().to_string_with_u_plus())
                            ".."
                            (block.code_range.end().to_string_with_u_plus())
                            " "
                            (format!("{}", block.block_name))
                        </a>
                    </li>
                }
            </ul>
        </div>
    })
}
