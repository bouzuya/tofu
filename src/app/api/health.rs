#[::topcoat::router::route(GET)]
async fn health() -> ::topcoat::Result<&'static str> {
    Ok("OK")
}
