#[ desc = "Library for implementing USB device drivers" ];
#[ license = "BSD" ];
#[ author = "Kevin Mehall" ];

#[ crate_type = "lib" ];
#[ feature(globs) ];
extern crate native;

pub use usb::*;

pub crate libusb;
pub crate usb;
