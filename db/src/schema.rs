table! {
    session_tokens (token) {
        user_id -> Uuid,
        token -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        accesso_id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

joinable!(session_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(session_tokens, users,);
