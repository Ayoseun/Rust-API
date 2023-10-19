// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        rewardsat -> Integer,
        link -> Text,
        creator_name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        address -> Text,
        balance -> Integer,
        badge -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
