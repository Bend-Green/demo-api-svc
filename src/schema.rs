table! {
    orgs (id) {
        id -> Bigint,
        name -> Varchar,
        description -> Text,
        url -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}
