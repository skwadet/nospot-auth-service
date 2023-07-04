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

    pub fn create_user(first_name: String, last_name: String, email: String) -> User {
        User {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            email,
            created_at: Utc::now(),
        }
    }
}