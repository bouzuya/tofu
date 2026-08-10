#[::topcoat::router::page]
async fn context(cx: &::topcoat::context::Cx) -> ::topcoat::Result {
    let count = &::topcoat::context::app_context::<crate::app::AppContext>(cx).count;
    let count = count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let parts = ::topcoat::router::parts(cx);
    let method = ::topcoat::router::method(cx);
    let uri = ::topcoat::router::uri(cx);
    let version = ::topcoat::router::version(cx);
    let headers = ::topcoat::router::headers(cx);
    let content_type = ::topcoat::router::content_type(cx);
    let extensions = ::topcoat::router::extensions(cx);
    ::topcoat::view::view! {
        <h1>"Context"</h1>
        <p>"count: "(format!("{:?}", count))</p>
        <p>"parts: "(format!("{:?}", parts))</p>
        <p>"method: "(format!("{:?}", method))</p>
        <p>"uri: "(format!("{:?}", uri))</p>
        <p>"version: "(format!("{:?}", version))</p>
        <p>"headers: "(format!("{:?}", headers))</p>
        <p>"content_type: "(format!("{:?}", content_type))</p>
        <p>"extensions: "(format!("{:?}", extensions))</p>
    }
}
