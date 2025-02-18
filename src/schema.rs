// @generated automatically by Diesel CLI.

diesel::table! {
    responses (id) {
        id -> Uuid,
        survey_id -> Nullable<Uuid>,
        user_id -> Uuid,
        answer -> Text,
        created_at -> Nullable<Timestamp>,
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
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        description -> Text,
    }
}

diesel::joinable!(responses -> surveys (survey_id));

diesel::allow_tables_to_appear_in_same_query!(
    responses,
    surveys,
    users,
);
