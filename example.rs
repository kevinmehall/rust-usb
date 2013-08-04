extern mod usb;
use usb::libusb;

fn main() {
	let c = usb::Context::new();
	c.setDebug(2);
	
	let devices = c.listDevices();

	foreach dev in devices.iter() {
		let desc = dev.descriptor();
		printfln!("Device %i.%i %04x:%04x",
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

					do spawn {
						printfln!("1 Opened device %?", handle1.ptr());
						printfln!("ctrl %?", handle1.ctrl_read(0xC0, 0x20, 0, 0, 64));
						printfln!("Write %?", handle1.write(0x02, libusb::LIBUSB_TRANSFER_TYPE_BULK, [1,2,3]));
						do handle1.write_stream(0x02, libusb::LIBUSB_TRANSFER_TYPE_BULK, 640, 8) |r| {
							match (r) {
								Ok(buf) => {
									printfln!("Write OK");
									buf[0] = 5;
								},
								Err(code) => {
									printfln!("Write error %?", code);
								}
							}
							true
						}

					}
					do spawn {
						printfln!("2 Opened device %?", handle2.ptr());
						printfln!("Read %?", handle2.read(0x81, libusb::LIBUSB_TRANSFER_TYPE_BULK, 64));
						do handle2.read_stream(0x81, libusb::LIBUSB_TRANSFER_TYPE_BULK, 640, 8) |r| {
							match (r) {
								Ok(buf) => printfln!("Read %?", buf.slice(0, 10)),
								Err(code) => printfln!("Read error %?", code)
							}
							true
						}
					}
				},
				Err(code) => {
					printfln!("Error opening device: %?", code);
				}
			}
		},
		None => println("Device not found"),
	}
}
