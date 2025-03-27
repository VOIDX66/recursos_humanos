// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        lastname -> Varchar,
        rol -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}
