use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, pin};
use viz::{server::conn::http1, Io, Request, Responder, Result, Router, Tree};

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) -> Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = TcpListener::bind(addr).await?;
        println!("listening on {addr}");

        let app = Router::new().get("/", index);
        let tree = Arc::new(Tree::from(app));

        loop {
            let (stream, addr) = listener.accept().await?;
            let tree = tree.clone();

            tokio::task::spawn(async move {
                // Pin the connection object so we can use tokio::select! below.
                let conn = http1::Builder::new()
                    .serve_connection(Io::new(stream), Responder::new(tree, Some(addr)));
                pin!(conn);

                tokio::select! {
                    res = conn.as_mut() => {
                        match res {
                            Ok(()) => println!("after polling conn, no error"),
                            Err(e) =>  println!("error serving connection: {e:?}"),
                        };
                        // break;
                    }
                }
            });
        }
    }
}

async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello, World!")
}
