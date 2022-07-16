use actix_web::{middleware::Logger, App, HttpServer};
use app::api::default_endpoint;
use env_logger::Env;
use log::info;
use paperclip::actix::OpenApiExt;

mod app;

const APP_PORT: u16 = 8080;

fn initialize_logger() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_api()
            .service(default_endpoint)
            .service(app::api::log::controller::get_logs)
            .wrap(Logger::default())
            .with_json_spec_at("/openapi.json")
            .with_swagger_ui_at("/docs")
            .build()
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    initialize_logger();
    info!("Starting web server at 0.0.0.0:{}", APP_PORT);
    run_server().await
}
