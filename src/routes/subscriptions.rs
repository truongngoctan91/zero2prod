use actix_web::{web, App, HttpResponse, HttpServer};

#[derive(serde::Deserialize)]
pub struct FormaDate {
    email: String,
    name: String,
}
pub async fn subscribe(
    _form: web::Form<FormaDate>,
    _connection: web::Data<PgConnection>
    ) -> HttpResponse {
    HttpResponse::Ok().finish()
}
