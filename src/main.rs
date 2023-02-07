use salvo::prelude::*;

#[handler]
async fn rust_server() -> &'static str {
    "Server http rust"
}
#[tokio::main]
async fn main() {
    let router = Router::new().get(rust_server);
    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(router)
        .await;
}
