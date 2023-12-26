// an example of bloating compile time caused by xitca-web api.
//
// *. this example may cause intense work to your IDE. try disable LSP and/or plugins to save
// power.

use xitca_web::{
    error::Error, handler::handler_service, http::WebResponse, service::Service, App, WebContext,
};

fn main() -> std::io::Result<()> {
    App::new()
        .at("/", handler_service(|| async { "Hello,World!" }))
        // compile time scale with the times enclosed_fn called.
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .enclosed_fn(middleware)
        .serve()
        .bind("localhost:8080")?
        .run()
        .wait()
}

async fn middleware<S>(service: &S, ctx: WebContext<'_>) -> Result<WebResponse, Error>
where
    S: for<'r> Service<WebContext<'r>, Response = WebResponse, Error = Error>,
{
    Box::pin(service.call(ctx)).await
}
