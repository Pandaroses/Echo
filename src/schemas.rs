use diesel::prelude::*;

table! {
    users {
        id -> VarChar,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        password -> Text
    }
}

table! {
    sessions {
        id -> VarChar,
        user_id -> VarChar
    }
}

joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(users , sessions);