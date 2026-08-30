use crate::components::breadcrumbs;

mod code_range;

#[::topcoat::router::page]
async fn get_blocks(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let app_context = ::topcoat::context::app_context::<crate::app::AppContext>(cx);
    ::topcoat::view::view! {
        <div class="page block-list-page">
            breadcrumbs(
                items: vec![
                    ("/", None, "tofu"),
                    ("/blocks", None, "Blocks")
                ]
            )
            <h1>"Blocks"</h1>
            <ul>
                for block in &app_context.blocks {
                    <li>
                        <a
                            href=(format!(
                            "/blocks/{}..{}", block.code_range.start()
                            .to_string_without_u_plus(), block.code_range.end()
                            .to_string_without_u_plus()
                        ))
                        >
                            (block
                                .code_range
                                .start()
                                .to_string_with_u_plus())
                            ".."
                            (block.code_range.end().to_string_with_u_plus())
                            " "
                            (format!("{}", block.block_name))
                        </a>
                    </li>
                }
            </ul>
        </div>
    }
}
