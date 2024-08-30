#![no_std]
//! Common definitions for USB constants.
//!
//! * [Universal Serial Bus Specification Revision 2.0](https://www.usb.org/document-library/usb-20-specification)
//! * [Universal Serial Bus 3.2 Specification](https://usb.org/document-library/usb-32-revision-11-june-2022)

mod descriptor;
pub use descriptor::{class_code, descriptor_type, language_id};

mod endpoint;
pub use endpoint::endpoint_attributes;

mod request;
pub use request::{feature_selector, request_type, standard_request, test_mode};

