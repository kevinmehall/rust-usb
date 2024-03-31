#![no_std]

mod descriptor;
pub use descriptor::{class, descriptor_type, language_id};

mod request;
pub use request::{feature_selector, standard_request, test_mode};
