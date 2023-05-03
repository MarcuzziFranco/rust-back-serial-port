use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| HttpResponse::Ok()))
            .route("/connect/{port}/{baud_rate}", web::get().to(connect))
            .route("/ports", web::get().to(ports_usb))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn connect(req: HttpRequest) -> impl Responder {
    let port = req.match_info().get("port").unwrap_or("default");
    let baud_rate = req.match_info().get("baud_rate").unwrap_or("9600");
    return format!("Connected port:{} with baud rate:{}", &port, &baud_rate);
}

async fn ports_usb(req: HttpRequest) -> impl Responder {
    let ports = "COM04";
    format!("{}", ports)
}
