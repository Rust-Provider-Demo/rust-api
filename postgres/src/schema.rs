// @generated automatically by Diesel CLI.

diesel::table! {
    providers (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        npi -> Varchar,
    }
}
