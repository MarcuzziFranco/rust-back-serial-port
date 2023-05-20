use actix_web::{http,get,post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::time::SystemTime;
use actix_cors::Cors;
use chrono::{DateTime, Utc};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::default()
                .allow_any_origin()
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT,
                    http::header::CONTENT_TYPE,
                ])
                .max_age(3600),)
            .service(ping)
            .service(echo) 
            .service(current_time)
            .route("/hey", web::get().to(manual_hello))
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

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("is life")
}

async fn ports_usb(req: HttpRequest) -> impl Responder {
    let ports = serialport::available_ports().expect("No ports founds");
    let mut vec_pors_names:Vec<String> = Vec::new();

    for p in ports {
       // println!("{}",p.port_name);
        vec_pors_names.push(p.port_name);
    }
    return web::Json(vec_pors_names);
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/time")]
async fn current_time() -> impl Responder{  
    let system_time = SystemTime::now(); 
    let date_time:DateTime<Utc> = system_time.into();
    let time_str:String = date_time.format("%d/%m/%Y %H:%M").to_string();
    HttpResponse::Ok().body(time_str)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
