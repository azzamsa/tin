use diesel::table;

table! {
    user_ (id) {
        id -> Int4,
        name -> Text,
        full_name -> Nullable<Text>,
    }
}
