use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

static LOCKED: AtomicBool = AtomicBool::new(false);

#[get("/wait")]
async fn wait() -> impl Responder {
    println!("Wait called");
    while LOCKED.load(Ordering::Relaxed) {
        println!("Waiting");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    println!("Waiting was a success");
    HttpResponse::Ok().body("Hello world!")
}

#[get("/lock")]
async fn lock() -> impl Responder {
    println!("Lock called");
    LOCKED.store(true, Ordering::Relaxed);
    HttpResponse::Ok().finish()
}

#[get("/unlock")]
async fn unlock() -> impl Responder {
    println!("Unlock called");
    LOCKED.store(false, Ordering::Relaxed);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(lock).service(unlock).service(wait))
        .bind(("0.0.0.0", 7700))?
        .run()
        .await
}
