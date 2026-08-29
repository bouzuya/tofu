#[::topcoat::view::component]
pub async fn character(c: char, thumbnail: bool) -> ::topcoat::Result {
    ::topcoat::view::view! {
        <span class=(::topcoat::view::class!("character", "thumbnail" if thumbnail))>
            (topcoat::view::Unescaped::new_unchecked(
                if c == ' ' { "&nbsp;".to_string() } else { c.to_string() },
            ))
        </span>
    }
}
