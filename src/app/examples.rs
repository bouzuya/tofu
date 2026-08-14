mod cookie;
mod login;
mod logout;

#[derive(Clone)]
struct User {
    #[allow(dead_code)]
    id: String,
}

#[allow(dead_code)]
async fn current_user(cx: &::topcoat::context::Cx) -> ::topcoat::Result<Option<User>> {
    let Some(hash) = ::topcoat::session::token_hash(cx).await? else {
        return Ok(None);
    };

    load_session_user(cx, &hash).await
}

#[allow(dead_code)]
async fn escalate(cx: &::topcoat::context::Cx) -> ::topcoat::Result<()> {
    if let Some(rotation) = ::topcoat::session::rotate(cx).await? {
        rekey_session(cx, &rotation.revoked, &rotation.session).await?;
    }
    Ok(())
}

#[allow(dead_code)]
async fn slide_expiration(cx: &::topcoat::context::Cx) -> ::topcoat::Result<()> {
    if let Some(session) = ::topcoat::session::refresh(cx).await? {
        update_session_expiry(cx, &session.token_hash, session.expires_at).await?;
    }
    Ok(())
}

async fn verify_credentials(_cx: &::topcoat::context::Cx) -> ::topcoat::Result<User> {
    Ok(User {
        id: "user123".to_string(),
    })
}

// dummy store
struct MySessionUserStore(
    std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<::topcoat::session::TokenHash, User>>,
    >,
);

async fn delete_session(
    cx: &::topcoat::context::Cx,
    hash: &::topcoat::session::TokenHash,
) -> ::topcoat::Result<()> {
    ::topcoat::context::app_context::<MySessionUserStore>(cx)
        .0
        .lock()
        .unwrap()
        .remove(hash);
    Ok(())
}

async fn load_session_user(
    cx: &::topcoat::context::Cx,
    hash: &::topcoat::session::TokenHash,
) -> ::topcoat::Result<Option<User>> {
    Ok(::topcoat::context::app_context::<MySessionUserStore>(cx)
        .0
        .lock()
        .unwrap()
        .get(hash)
        .cloned())
}

async fn persist_session(
    cx: &::topcoat::context::Cx,
    user: &User,
    session: &::topcoat::session::Session,
) -> ::topcoat::Result<()> {
    ::topcoat::context::app_context::<MySessionUserStore>(cx)
        .0
        .lock()
        .unwrap()
        .insert(session.token_hash.clone(), user.clone());
    Ok(())
}

async fn rekey_session(
    cx: &::topcoat::context::Cx,
    revoked: &::topcoat::session::TokenHash,
    session: &::topcoat::session::Session,
) -> ::topcoat::Result<()> {
    if let Some(user) = load_session_user(cx, revoked).await? {
        persist_session(cx, &user, session).await?;
        delete_session(cx, revoked).await?;
    }
    Ok(())
}

async fn update_session_expiry(
    cx: &::topcoat::context::Cx,
    hash: &::topcoat::session::TokenHash,
    expires_at: std::time::SystemTime,
) -> ::topcoat::Result<()> {
    if let Some(user) = load_session_user(cx, hash).await? {
        let session = ::topcoat::session::Session {
            token_hash: hash.clone(),
            expires_at,
        };
        persist_session(cx, &user, &session).await?;
    }
    Ok(())
}
