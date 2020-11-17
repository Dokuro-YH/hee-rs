use tide_tracing::TraceMiddleware;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    tracing_log::LogTracer::init()?;

    let mut app = tide::new();
    // middleware
    app.with(TraceMiddleware::new());

    // routes
    app.at("/").all(|_req| async { Ok("Hello, world!") });

    // start
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
