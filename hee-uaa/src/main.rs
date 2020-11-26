mod uaa;

use uaa::Uaa;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let uaa = Uaa::new();
    let config = uaa.config();
    let addr = format!("{}:{}", config.address, config.port);

    // tide
    let mut app = tide::new();

    // routes
    app.at("/").all(|_req| async { Ok("Hello, world!") });

    app.listen(addr).await?;
    Ok(())
}
