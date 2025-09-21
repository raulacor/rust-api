use actix_web::{
    HttpResponse, Responder, get,
    web::{ServiceConfig, scope},
};
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "health checker is running. . .";

    HttpResponse::Ok().json(json!({
    "status": "success",
    "message": MESSAGE
    }))
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api").service(health_checker);
    conf.service(scope);
}
