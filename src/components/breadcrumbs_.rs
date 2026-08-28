#[::topcoat::view::component]
pub async fn breadcrumbs(items: Vec<(&str, &str)>) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <nav class="breadcrumb-list">
            <ol>
                for (href, text) in items {
                    <li><a href=(href)>(text)</a></li>
                }
            </ol>
        </nav>
    }
}
