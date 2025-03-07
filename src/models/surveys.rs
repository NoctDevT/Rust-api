use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::{surveys, questions, responses};

#[derive(Queryable, Serialize)]
#[diesel(table_name = surveys)]
pub struct Survey {
    pub id: Uuid,
    pub title: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = surveys)]
pub struct NewSurvey {
    pub title: String,
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = questions)]
pub struct Question {
    pub id: Uuid,
    pub survey_id: Uuid,
    pub question_text: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = questions)]
pub struct NewQuestion {
    pub survey_id: Uuid,
    pub question_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SurveyResponse {
    pub survey_id: Uuid,
    pub title: String,
    pub questions: Vec<String>, 
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = responses)]
pub struct Response {
    pub id: Uuid,
    pub survey_id: Option<Uuid>,
    pub question_id: Uuid, 
    pub user_id: Uuid,
    pub answer: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = responses)]
pub struct NewResponse {
    pub survey_id: Option<Uuid>,
    pub question_id: Uuid, 
    pub answer: String,
    #[serde(skip_deserializing)] // Need this so that ok(response) == json_from_str<newResponse>(text) will work without userid being specified`
    pub user_id: Option<Uuid>,
}


#[derive(Debug, Deserialize)]
#[serde(tag = "type", content="data")]
pub enum ClientMessage {
    Authenticate { token: String },
    RequestSurvey {survey_id: Uuid},
    SubmitResponses(SubmitResponsesData), 
    Ping,
}

#[derive(Debug, Deserialize)]
pub struct SubmitResponsesData {
    pub survey_id: Uuid,
    pub responses: Vec<QuestionResponse>,
}


#[derive(Debug, Deserialize)]
pub struct QuestionResponse {
    pub question_id: Uuid,
    pub answer: String,
}
