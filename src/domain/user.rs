pub mod user {
    use uuid::Uuid;
    use chrono::DateTime;
    use chrono::Utc;
    use regex::Regex;

    pub struct User {
        pub id: Uuid,
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub created_at: DateTime<Utc>,
    }

    pub fn validate_email(email: &String) -> bool {
        Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-.][a-z0-9]+)*\.[a-z]{2,6})")
            .unwrap()
            .is_match(email)
    }
}