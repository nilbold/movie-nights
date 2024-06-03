use std::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    movie_nights::run(listener)?.await?;

    Ok(())
}
