#[link(name = "usb", vers = "0.1", author = "Kevin Mehall")];

#[ desc = "Library for implementing USB device drivers" ];
#[ license = "BSD" ];
#[ author = "Kevin Mehall" ];

#[ crate_type = "lib" ];
#[ feature(globs) ];

pub use usb::*;

pub mod libusb;
pub mod usb;
