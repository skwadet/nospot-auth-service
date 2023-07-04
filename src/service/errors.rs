pub mod internal_errors {
    pub struct ServiceError {
        pub repr_api: String,
        pub repr_internal: String,
        pub code: u32,
    }

    pub fn new_service_error(repr_api: String, repr_internal: String, code: u32) -> ServiceError {
        ServiceError {
            repr_api,
            repr_internal,
            code,
        }
    }
}