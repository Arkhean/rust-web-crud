use diesel::prelude::*;

use crate::models::{self, NoteSerializer};

type DbError = Box<dyn std::error::Error + Send + Sync>;

// *******************************************************************

pub fn get_by_id(
    conn: &mut SqliteConnection,
    note_id: i32,
) -> Result<Option<models::NoteSerializer>, DbError> {
    use crate::schemas::notes::dsl::*;
    let note = notes
        .filter(id.eq(note_id))
        .first::<models::NoteSerializer>(conn)
        .optional()?;
    Ok(note)
}

// *******************************************************************

pub fn create(
    conn: &mut SqliteConnection,
    new_note: &models::NoteCreator,
) -> Result<models::NoteSerializer, DbError> {
    use crate::schemas::notes::dsl::*;
    let result = diesel::insert_into(notes)
        .values(new_note)
        .get_result::<(i32, String, String)>(conn)?;
    Ok(models::NoteSerializer {
        id: result.0,
        title: new_note.title.to_owned(),
        text: new_note.text.to_owned(),
    })
}

// *******************************************************************

pub fn delete(conn: &mut SqliteConnection, note_id: i32) -> Result<usize, DbError> {
    use crate::schemas::notes::dsl::*;
    let result = diesel::delete(notes.filter(id.eq(note_id))).execute(conn)?;
    Ok(result)
}

// *******************************************************************

pub fn update(
    conn: &mut SqliteConnection,
    note_id: i32,
    new_note: &models::NoteCreator,
) -> Result<Option<models::NoteSerializer>, DbError> {
    use crate::schemas::notes::dsl::*;
    let result = diesel::update(notes.filter(id.eq(note_id)))
        .set((
            title.eq(new_note.title.to_owned()),
            text.eq(new_note.text.to_owned()),
        ))
        .get_result::<(i32, String, String)>(conn)
        .optional()?;
    Ok(match result {
        Some(r) => Some(models::NoteSerializer {
            id: r.0,
            title: new_note.title.to_owned(),
            text: new_note.text.to_owned(),
        }),
        None => None,
    })
}

// *******************************************************************

pub fn search(conn: &mut SqliteConnection) -> Result<Vec<models::NoteSerializer>, DbError> {
    use crate::schemas::notes::dsl::*;
    let records: Vec<NoteSerializer> = notes
        .select(models::NoteSerializer::as_select())
        .load(conn)?;
    Ok(records)
}
