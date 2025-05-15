// @generated automatically by Diesel CLI.

diesel::table! {
    applications (id) {
        id -> Varchar,
        user_id -> Varchar,
        vacancy_id -> Varchar,
        application_date -> Nullable<Timestamptz>,
        #[max_length = 20]
        status -> Varchar,
        comment -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    evaluations (id) {
        id -> Varchar,
        vacancy_id -> Varchar,
        candidate_id -> Varchar,
        evaluator_id -> Varchar,
        evaluation_date -> Timestamptz,
        feedback -> Nullable<Text>,
        score -> Nullable<Numeric>,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    trainings (id) {
        id -> Varchar,
        employee_id -> Varchar,
        trainer_id -> Nullable<Varchar>,
        #[max_length = 50]
        training_type -> Varchar,
        training_date -> Timestamptz,
        feedback -> Nullable<Text>,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Varchar,
        #[max_length = 15]
        id_number -> Varchar,
        #[max_length = 30]
        name -> Varchar,
        #[max_length = 30]
        lastname -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        password -> Varchar,
        #[max_length = 20]
        role -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    vacancies (id) {
        id -> Varchar,
        title -> Text,
        description -> Text,
        requirements -> Nullable<Text>,
        salary -> Nullable<Numeric>,
        opening_date -> Date,
        closing_date -> Nullable<Date>,
        #[max_length = 20]
        status -> Varchar,
        created_by -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(applications -> users (user_id));
diesel::joinable!(applications -> vacancies (vacancy_id));
diesel::joinable!(evaluations -> vacancies (vacancy_id));
diesel::joinable!(vacancies -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    applications,
    evaluations,
    trainings,
    users,
    vacancies,
);
