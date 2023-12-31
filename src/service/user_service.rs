pub mod user_service {
    use crate::{
        service::errors::internal_errors::{ServiceError, new_service_error},
        service::user_service::user_operations,
        domain::user::user::{User, create_user},
    };

    use uuid::Uuid;
    use chrono::DateTime;
    use chrono::Utc;
    use chrono::offset;
    use std::result::Result;
    use user_operations::validate_email;


    pub fn add_user(first_name: String, last_name: String, email: String) -> Result<User, ServiceError> {
        match validate_email(&email) {
            true => Ok(create_user(first_name, last_name, email)),
            _ => Err(new_service_error("Неправильный email!".to_string(),
                                       "Couldn't create user!".to_string(),
                                       400))
        }
    }
}

pub mod user_operations {
    use regex::Regex;

    pub fn validate_email(email: &String) -> bool {
        Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-.][a-z0-9]+)*\.[a-z]{2,6})")
            .unwrap()
            .is_match(email)
    }
}