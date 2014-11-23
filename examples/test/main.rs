extern crate usb;
use usb::libusb;

fn main() {
	let c = usb::Context::new();
	c.set_debug(2);

	let devices = c.list_devices();

	for dev in devices.iter() {
		let desc = dev.descriptor();
		println!("Device {:03}.{:03} {:04x}:{:04x}",
			dev.bus(),
			dev.address(),
			desc.idVendor as uint,
			desc.idProduct as uint
		);
	}

	match c.find_by_vid_pid(0x59e3, 0x0a23) {
		Some(dev) => {
			match dev.open() {
				Ok(handle) => {
					let handle1 = handle.clone();
					let handle2 = handle.clone();

					std::task::spawn(proc() {
						println!("1 Opened device {}", handle1.ptr());
						println!("ctrl {}", handle1.ctrl_read(0xC0, 0x20, 0, 0, 64));
						println!("Write {}", handle1.write(0x02, libusb::LIBUSB_TRANSFER_TYPE_BULK, &[1,2,3]));
						handle1.write_stream(0x02, libusb::LIBUSB_TRANSFER_TYPE_BULK, 640, 8, |r| {
							match r {
								Ok(buf) => {
									println!("Write OK");
									buf[0] = 5;
								},
								Err(code) => {
									println!("Write error {}", code);
								}
							}
							true
						});

					});
					std::task::spawn(proc() {
						println!("2 Opened device {}", handle2.ptr());
						println!("Read {}", handle2.read(0x81, libusb::LIBUSB_TRANSFER_TYPE_BULK, 64));
						handle2.read_stream(0x81, libusb::LIBUSB_TRANSFER_TYPE_BULK, 640, 8, |r| {
							match r {
								Ok(buf) => println!("Read {}", buf.slice(0, 10)),
								Err(code) => println!("Read error {}", code)
							}
							true
						});
					});
				},
				Err(code) => {
					println!("Error opening device: {}", code);
				}
			}
		},
		None => println!("Device not found"),
	}
}
