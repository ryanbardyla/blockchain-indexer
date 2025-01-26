use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct PriceResponse {
    pair: String,
    price: f64,
}

#[get("/price/{pair}")]
async fn get_price(path: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    let pair = path.into_inner();

    let price: (f64,) = sqlx::query_as(
        "SELECT price FROM price_feeds 
         WHERE pair = $1 
         ORDER BY block_number DESC 
         LIMIT 1",
    )
    .bind(&pair)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or_default();

    HttpResponse::Ok().json(PriceResponse {
        pair,
        price: price.0,
    })
}

pub async fn start_api_server(pool: PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_price)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
