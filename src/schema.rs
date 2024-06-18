// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Nullable<Integer>,
        title -> Nullable<Text>,
        text -> Nullable<Text>,
    }
}
