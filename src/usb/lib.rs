#[ desc = "Library for implementing USB device drivers" ];
#[ license = "BSD" ];
#[ author = "Kevin Mehall" ];

#[ crate_type = "lib" ];
#[ feature(globs) ];
extern mod native;

pub use usb::*;

pub mod libusb;
pub mod usb;
