#[::topcoat::view::component]
pub async fn character(c: char, thumbnail: bool) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <span
            style=(if thumbnail {
                "align-items: center; border: 2px solid #000; display: inline-flex; flex-flow: column nowrap; font-size: 16px; height: 24px; justify-content: center; width: 24px;"
            } else {
                "align-items: center; border: 2px solid #000; display: flex; flex-flow: column nowrap; font-size: 128px; height: 160px; justify-content: center; width: 160px;"
            })
        >
            (topcoat::view::Unescaped::new_unchecked(
                if c == ' ' { "&nbsp;".to_string() } else { c.to_string() },
            ))
        </span>
    }
}
