pub mod user {
    use uuid::Uuid;
    use chrono::DateTime;
    use chrono::Utc;

    pub struct User {
        pub id: Uuid,
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub created_at: DateTime<Utc>,
    }
}