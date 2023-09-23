use viz::Result;
use y_dash::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::new();

    server.run().await
}
