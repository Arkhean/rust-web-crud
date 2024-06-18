use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder};

use crate::models::NoteCreator;
use crate::{service, DbPool};

#[get("")]
async fn get_notes(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let records = web::block(move || {
        let mut conn = pool.get()?;
        service::search(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(records))
}

#[get("/{note_id}")]
async fn get_note_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let note_id = path.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let record = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;
        service::get_by_id(&mut conn, note_id)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match record {
        // record was found; return 200 response with JSON formatted user object
        Some(record) => HttpResponse::Ok().json(record),
        // record was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No note found with ID: {note_id}")),
    })
}

#[post("")]
async fn create_note(
    pool: web::Data<DbPool>,
    body: web::Json<NoteCreator>,
) -> actix_web::Result<impl Responder> {
    let note = body.into_inner();
    let record = web::block(move || {
        let mut conn = pool.get()?;
        service::create(&mut conn, &note)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(record))
}

#[put("/{note_id}")]
async fn edit_note(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body: web::Json<NoteCreator>,
) -> actix_web::Result<impl Responder> {
    let note_id = path.into_inner();
    let note = body.into_inner();
    // first get record to check that it exists
    let record = web::block(move || {
        let mut conn = pool.get()?;
        service::update(&mut conn, note_id, &note)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(match record {
        Some(record) => HttpResponse::Ok().json(record),
        None => HttpResponse::NotFound().body(format!("No note found with ID: {note_id}")),
    })
}

#[delete("/{note_id}")]
async fn delete_note(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let note_id = path.into_inner();
    let result = web::block(move || {
        let mut conn = pool.get()?;
        service::delete(&mut conn, note_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(match result {
        1 => HttpResponse::NoContent().finish(),
        _ => HttpResponse::NotFound().body(format!("No note found with ID: {note_id}")),
    })
}

// router config ***********************************************************

pub fn notes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notes")
            .service(get_notes)
            .service(get_note_by_id)
            .service(create_note)
            .service(edit_note)
            .service(delete_note),
    );
}
