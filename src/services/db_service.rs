use diesel::prelude::*;
use crate::models::user::User;
use crate::models::surveys::{Survey, Question, SurveyResponse, NewResponse}; 
use diesel::result::Error;
use crate::schema::users::dsl::*; 
use crate::schema::responses::dsl::*; 
use uuid::Uuid;

pub fn get_user_id_by_username(conn: &mut PgConnection, username_str: &str) -> Result<Uuid, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    users
        .filter(username.eq(username_str)) 
        .select(id) 
        .first::<Uuid>(conn) 
}

pub fn get_user_by_username( conn: &mut PgConnection, username_query: &str, ) 
-> Result<User, diesel::result::Error> 
{
    users
        .filter(username.eq(username_query))
        .select(User::as_select())
        .first::<User>(conn)}

pub fn fetch_survey_with_questions(conn: &mut PgConnection) -> Result<SurveyResponse, Error> {
    use crate::schema::surveys::dsl::*;
    use crate::schema::questions::dsl::*;

    let survey = surveys.order(created_at.desc()).first::<Survey>(conn)?;

    let question_list: Vec<Question> = questions
        .filter(survey_id.eq(survey.id))
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