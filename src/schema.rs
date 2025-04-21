// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Int4,
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
        rol -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
