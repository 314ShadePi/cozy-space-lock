use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(PartialEq, Eq, Copy, Clone)]
enum CStatus {
    Locked,
    UnLocked,
}

impl std::fmt::Display for CStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CStatus::Locked => write!(f, "LOCKED"),
            CStatus::UnLocked => write!(f, "UNLOCKED"),
        }
    }
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

async fn get_status(config: web::Data<Arc<Mutex<CStatus>>>) -> CStatus {
    *config.lock().await
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
    let c_status = get_status(config).await.to_string();
    println!("RING - {}", c_status.clone());
    HttpResponse::Ok().body(c_status)
}

#[get("/status")]
async fn status(config: web::Data<Arc<Mutex<CStatus>>>) -> impl Responder {
    let c_status = get_status(config).await.to_string();
    println!("STATUS - {}", c_status.clone());
    HttpResponse::Ok().body(c_status)
}
