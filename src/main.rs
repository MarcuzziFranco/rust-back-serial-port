use salvo::prelude::*;

#[tokio::main]
async fn main() {
    //let router = Router::new().get(rust_server);

    let router = Router::with_path("serial-data")
        .get(status_server)
        .push(Router::with_path("create-connection").get(create_connection))
        .push(Router::with_path("ports").get(port_usb_active));

    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(router)
        .await;
}

#[handler]
async fn status_server() -> &'static str {
    "Server rust is life"
}

#[handler]
async fn port_usb_active() -> &'static str {
    "Port usb active"
}

#[handler]
async fn create_connection() -> &'static str {
    "Connection created"
}
