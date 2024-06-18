#[macro_use]
extern crate diesel;

use crate::router::notes_config;
use actix_web::web::PathConfig;
use actix_web::{error, http::header, middleware, web, App, HttpResponse, HttpServer, HttpRequest};
use diesel::{prelude::*, r2d2};

mod models;
mod router;
mod schemas;
mod service;

/*
 * TODO
 * - write tests
 * - add comments
 * - generate openapi
 */

// main ************************************************************

/// Short-hand for the database pool type to use throughout the app.
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            // configure path error handler
            .app_data(PathConfig::default().error_handler(path_error_handler))
            // add request logger middleware
            .wrap(middleware::Logger::default())
            // add router
            .configure(notes_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn initialize_db_pool() -> DbPool {
    let conn_spec = "test.db";
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

fn path_error_handler(err: error::PathError, _req: &HttpRequest) -> error::Error {
    let error_text = err.to_string();
    let response = HttpResponse::UnprocessableEntity()
        .append_header(header::ContentType::json())
        .body(format!(
            "{{\"detail\": \"{}\"}}",
            error_text.escape_default()
        ));
    error::InternalError::from_response(err, response).into()
}
