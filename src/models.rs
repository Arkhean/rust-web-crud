use serde::{Deserialize, Serialize};

use crate::schemas::notes;

#[derive(Debug, Clone, Serialize, Queryable, Selectable)]
#[diesel(table_name = notes)]
pub struct NoteSerializer {
    pub id: i32,
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = notes)]
pub struct NoteCreator {
    pub title: String,
    pub text: String,
}
