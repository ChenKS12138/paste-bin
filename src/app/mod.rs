mod assets;
mod config;
mod dao;
mod entity;
mod service;
mod state;
mod template;

use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpResponse, HttpServer};

const FORM_LIMIT: usize = 100 * 1024 * 1024;

#[actix_web::main]
pub async fn run(bind: String, redis_client: redis::Client) -> std::io::Result<()> {
    let connection = redis_client.get_connection().unwrap();
    let state = web::Data::new(state::AppState {
        connection: Arc::new(Mutex::new(connection)),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(web::FormConfig::default().limit(FORM_LIMIT))
            .service(service::index::index)
            .service(service::public::index)
            .service(service::paste::create)
            .service(service::paste::query)
            .service(service::theme::index)
            .default_service(web::to(|| HttpResponse::NotFound().body("Not Found")))
    })
    .bind(&bind)?
    .run()
    .await
}
