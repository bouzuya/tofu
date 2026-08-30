#[::topcoat::view::component]
pub async fn breadcrumbs(
    items: Vec<(&str, Option<::topcoat::asset::Asset>, &str)>,
) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <nav class="breadcrumb-list">
            <ol>
                for (href, asset, text) in items {
                    <li>
                        <a href=(href)>
                            match asset {
                                Some(asset) => <span class="icon">
                                    <img alt="" height="24" src=(asset) width="24" />
                                </span>,
                                None => "",
                            }
                            <span class="text">(text)</span>
                        </a>
                    </li>
                }
            </ol>
        </nav>
    }
}
