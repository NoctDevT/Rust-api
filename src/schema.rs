// @generated automatically by Diesel CLI.

diesel::table! {
    questions (id) {
        id -> Uuid,
        survey_id -> Uuid,
        question_text -> Text,
    }
}

diesel::table! {
    responses (id) {
        id -> Uuid,
        survey_id -> Nullable<Uuid>,
        user_id -> Uuid,
        answer -> Text,
        created_at -> Nullable<Timestamp>,
        question_id -> Uuid,
    }
}

diesel::table! {
    surveys (id) {
        id -> Uuid,
        title -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        description -> Text,
        id -> Uuid,
    }
}

diesel::joinable!(questions -> surveys (survey_id));
diesel::joinable!(responses -> questions (question_id));
diesel::joinable!(responses -> surveys (survey_id));

diesel::allow_tables_to_appear_in_same_query!(
    questions,
    responses,
    surveys,
    users,
);
