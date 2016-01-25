rust-usb 
========================

libusb bindings for rust

MIT Licensed

## Deprecated: see [libusb-rs](https://github.com/dcuddeback/libusb-rs)

This library was written in 2013 in a language that was called Rust, but was very different than the language we know as Rust 1.0+. Specifically, it had green-threaded IO, and it lacked today's conventions around error handling. This library has been updated along the way just enough to keep compiling, but would effectively be a different library if written in modern Rust.

@dcuddeback has started to write [that library](https://github.com/dcuddeback/libusb-rs).

A future possibility for the `usb` crate name could be to implement common USB patterns on top of the `libusb` crate and Libusb itself, such as: 
  * finding devices and interfaces with descriptors that match certain criteria
  * maximizing streaming throughput by stacking asynchronous transfers
  * delimiting logical frame boundaries with zero-length packets
