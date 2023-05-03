use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| HttpResponse::Ok()))
            .route("/ports", web::get().to(ports_usb))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn ports_usb(req: HttpRequest) -> impl Responder {
    let ports = "COM04";
    format!("{}", ports)
}
