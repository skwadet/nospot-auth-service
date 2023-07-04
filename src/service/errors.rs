pub mod internal_errors {
    pub struct ServiceError {
        pub repr_api: String,
        pub repr_internal: String,
        pub code: u32,
    }
}