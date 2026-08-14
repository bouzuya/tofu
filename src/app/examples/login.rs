#[::topcoat::router::route(POST)]
async fn login(
    cx: &::topcoat::context::Cx,
) -> ::topcoat::Result<::topcoat::router::error::SeeOther> {
    let user = super::verify_credentials(cx).await?;

    let session = ::topcoat::session::start(cx).await?;
    super::persist_session(cx, &user, &session).await?;

    Ok(::topcoat::router::error::see_other("/"))
}
