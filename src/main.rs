use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(PartialEq, Eq)]
enum CStatus {
    Locked,
    UnLocked,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = web::Data::new(Arc::new(Mutex::new(CStatus::Locked)));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"]);

        App::new()
            .wrap(cors)
            .app_data(config.clone())
            .service(unlock)
            .service(lock)
            .service(ring)
            .service(status)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/unlock")]
async fn unlock(config: web::Data<Arc<Mutex<CStatus>>>) -> impl Responder {
    let mut c_lock = config.lock().await;
    *c_lock = CStatus::UnLocked;
    println!("UNLOCKED");
    HttpResponse::Ok().body("UNLOCKED")
}

#[get("/lock")]
async fn lock(config: web::Data<Arc<Mutex<CStatus>>>) -> impl Responder {
    let mut c_lock = config.lock().await;
    *c_lock = CStatus::Locked;
    println!("LOCKED");
    HttpResponse::Ok().body("LOCKED")
}

#[get("/ring")]
async fn ring(config: web::Data<Arc<Mutex<CStatus>>>) -> impl Responder {
    let c_lock = config.lock().await;

    if *c_lock == CStatus::Locked {
        println!("RING - LOCKED");
        HttpResponse::Ok().body("LOCKED")
    } else {
        println!("RING - UNLOCKED");
        HttpResponse::Ok().body("UNLOCKED")
    }
}

#[get("/status")]
async fn status(config: web::Data<Arc<Mutex<CStatus>>>) -> impl Responder {
    let c_lock = config.lock().await;

    if *c_lock == CStatus::Locked {
        println!("STATUS - LOCKED");
        HttpResponse::Ok().body("LOCKED")
    } else {
        println!("STATUS - UNLOCKED");
        HttpResponse::Ok().body("UNLOCKED")
    }
}
