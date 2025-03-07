use diesel::prelude::*;
use crate::models::user::User;
use crate::models::surveys::{Survey, Question, SurveyResponse, NewResponse}; 
use diesel::result::Error;
use uuid::Uuid;

use crate::schema::users::dsl::*;
use crate::schema::responses::dsl::*;
use crate::schema::surveys::dsl::{surveys, id as survey_uuid, created_at};
use crate::schema::questions::dsl::{questions, survey_id as question_survey_id};
use crate::schema::users::dsl::{users, username, id as user_id};

use redis::{Commands, RedisResult};
use serde_json;

pub fn get_user_id_by_username(conn: &mut PgConnection, username_str: &str) -> Result<Uuid, diesel::result::Error> {
    users
        .filter(username.eq(username_str)) 
        .select(user_id)
        .limit(1) 
        .first::<Uuid>(conn) 
}

pub fn get_user_by_username(conn: &mut PgConnection, username_query: &str) -> Result<User, diesel::result::Error> {
    users
        .filter(username.eq(username_query))
        .select(User::as_select())
        .first::<User>(conn)
}

pub fn fetch_survey_with_questions(conn: &mut PgConnection) -> Result<SurveyResponse, Error> {
    let survey = surveys.order(created_at.desc()).first::<Survey>(conn)?;

    let question_list: Vec<Question> = questions
        .filter(question_survey_id.eq(survey.id))
        .load::<Question>(conn)?;

    Ok(SurveyResponse {
        survey_id: survey.id,
        title: survey.title.clone(),
        questions: question_list.into_iter().map(|q| q.question_text).collect(),
    })
}

pub fn fetch_survey_with_id(conn: &mut PgConnection, survey_id_param: Uuid) -> Result<SurveyResponse, Error> {
    let survey = surveys.filter(survey_uuid.eq(survey_id_param)).first::<Survey>(conn)?;

    let question_list: Vec<Question> = questions
        .filter(question_survey_id.eq(survey.id))
        .load::<Question>(conn)?;

    Ok(SurveyResponse {
        survey_id: survey.id,
        title: survey.title.clone(),
        questions: question_list.into_iter().map(|q| q.question_text).collect(),
    })
}

pub fn store_survey_response(conn: &mut PgConnection, response: NewResponse) -> bool {
    diesel::insert_into(responses)
        .values(&response)
        .execute(conn)
        .is_ok()
}

//DB optimisations
// Redis cached queries


// pub fn cache_survey(redis_conn: &mut redis::Connection, survey_id: Uuid, survey: &SurveyResponse) -> RedisResult<()> {
//     let key = format!("survey:{}", survey_id);
//     let survey_json = serde_json::to_string(survey).unwrap();
//     redis_conn.set_ex(key, survey_json, 600)?; 
//     Ok(())
// }

// pub fn get_cached_survey(redis_conn: &mut redis::Connection, survey_id: Uuid) -> Option<SurveyResponse> {
//     let key = format!("survey:{}", survey_id);
//     let survey_json: RedisResult<String> = redis_conn.get(key);
//     survey_json.ok().and_then(|json| serde_json::from_str(&json).ok())
// }

