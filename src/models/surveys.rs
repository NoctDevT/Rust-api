use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::{surveys, responses};

// SURVEYS

#[derive(Queryable, Serialize)]
#[diesel(table_name = surveys)]
pub struct Survey {
    pub id: Uuid,
    pub title: String,
    // Because in schema.rs, created_at -> Nullable<Timestamp>,
    // we need an Option<NaiveDateTime>
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = surveys)]
pub struct NewSurvey {
    pub title: String,
}

// RESPONSES

#[derive(Queryable, Serialize)]
#[diesel(table_name = responses)]
pub struct Response {
    pub id: Uuid,
    pub survey_id: Option<Uuid>,
    pub user_id: Uuid,
    pub answer: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = responses)]
pub struct NewResponse {
    pub survey_id: Option<Uuid>,
    pub user_id: Uuid,
    pub answer: String,
}
