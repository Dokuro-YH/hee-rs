use tracing_subscriber::{filter, fmt::time};

#[async_std::main]
async fn main() -> tide::Result<()> {
    // tracing
    let filter = filter::EnvFilter::from_default_env();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_timer(time::ChronoLocal::with_format("%FT%T%.3f%:z".to_string()))
        .init();

    // tide
    let mut app = tide::new();

    // routes
    app.at("/").all(|_req| async { Ok("Hello, world!") });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
