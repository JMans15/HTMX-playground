use anyhow::Result;

use actix_files::NamedFile;
use actix_web::{middleware, web, App, HttpRequest, HttpServer};

mod http_services;
use http_services::*;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(index)
            .service(loading_text)
            .service(default_view)
            .service(load_image)
            .route("static/image.png", web::get().to(get_image))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

async fn get_image(_req_: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/bigimage.png")?)
}
