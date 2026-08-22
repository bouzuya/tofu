mod code_range;

#[::topcoat::router::page]
async fn get_blocks(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let app_context = ::topcoat::context::app_context::<crate::app::AppContext>(cx);
    ::topcoat::view::view! {
        <nav class="breadcrumb-list">
            <ol>
                <li><a href="/">"Home"</a></li>
                <li><a href="/blocks">"Blocks"</a></li>
            </ol>
        </nav>
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
                        (block.code_range.start().to_string_with_u_plus())
                        ".."
                        (block.code_range.end().to_string_with_u_plus())
                        " "
                        (format!("{}", block.block_name))
                    </a>
                </li>
            }
        </ul>
    }
}
