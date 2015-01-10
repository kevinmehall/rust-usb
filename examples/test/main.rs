#![allow(unstable)]

extern crate usb;
use usb::libusb;

use std::thread::{Thread};

fn main() {
	let c = usb::Context::new();
	c.set_debug(2);

	let devices = c.list_devices();

	for dev in devices.iter() {
		let desc = dev.descriptor();
		println!("Device {:03}.{:03} {:04x}:{:04x}",
			dev.bus(),
			dev.address(),
			desc.idVendor,
			desc.idProduct
		);
	}

	match c.find_by_vid_pid(0x59e3, 0x0a23) {
		Some(dev) => {
			match dev.open() {
				Ok(handle) => {
					let handle1 = handle.clone();
					let handle2 = handle.clone();

					let t1 = Thread::scoped(move || {
						println!("1 Opened device {:?}", handle1.ptr());
						println!("ctrl {:?}", handle1.ctrl_read(0xC0, 0x20, 0, 0, 64, 0));
						println!("Write {:?}", handle1.write(0x02, libusb::LIBUSB_TRANSFER_TYPE_BULK, &[1,2,3], 0));
						handle1.write_stream(0x02, libusb::LIBUSB_TRANSFER_TYPE_BULK, 640, 8, &mut |r| {
							match r {
								Ok(buf) => {
									println!("Write OK");
									buf[0] = 5;
								},
								Err(code) => {
									println!("Write error {:?}", code);
								}
							}
							true
						});
					});
					let t2 = Thread::scoped(move || {
						println!("2 Opened device {:?}", handle2.ptr());
						println!("Read {:?}", handle2.read(0x81, libusb::LIBUSB_TRANSFER_TYPE_BULK, 64, 0));
						handle2.read_stream(0x81, libusb::LIBUSB_TRANSFER_TYPE_BULK, 640, 8, &mut |r| {
							match r {
								Ok(buf) => println!("Read {:?}", buf.slice(0, 10)),
								Err(code) => println!("Read error {:?}", code)
							}
							true
						});
					});

					t1.join().ok().unwrap();
					t2.join().ok().unwrap();
				},
				Err(code) => {
					println!("Error opening device: {:?}", code);
				}
			}
		},
		None => println!("Device not found"),
	}
}
