pub mod user_service {
    use uuid::Uuid;
    use chrono::DateTime;
    use chrono::Utc;
    use chrono::offset;
    use regex::Regex;
    use std::result::Result;
    use crate::{
        service::errors::internal_errors::ServiceError,
        domain::user::user::{User, validate_email},
    };

    pub fn create_user(first_name: String, last_name: String, email: String) -> Result<User, ServiceError> {
        match validate_email(&email) {
            true => Ok(User {
                id: Uuid::new_v4(),
                first_name,
                last_name,
                email,
                created_at: Utc::now(),
            }),
            _ => Err(ServiceError { repr: "Couldn't create user!".to_string() })
        }
    }
}