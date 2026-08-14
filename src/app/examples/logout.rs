#[::topcoat::router::route(POST)]
async fn logout(
    cx: &::topcoat::context::Cx,
) -> ::topcoat::Result<::topcoat::router::error::SeeOther> {
    if let Some(hash) = ::topcoat::session::stop(cx).await? {
        super::delete_session(cx, &hash).await?;
    }
    Ok(::topcoat::router::error::see_other("/"))
}
