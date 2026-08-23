pub struct CreateUser {
    pub name: String,
}

pub struct User {
    pub id: String,
}

#[::topcoat::router::route(POST)]
async fn create_user(
    cx: &::topcoat::context::Cx,
    ::topcoat::router::content::Json(input): ::topcoat::router::content::Json<CreateUser>,
) -> ::topcoat::Result<::topcoat::router::content::Json<User>> {
    Ok(::topcoat::router::content::Json(User {
        id: "1".to_string(),
    }))
}
