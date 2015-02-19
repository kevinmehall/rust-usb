

extern crate usb;
use usb::libusb;

use std::thread;

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

	let dev = c.find_by_vid_pid(0x59e3, 0x0a23).expect("Device not found");
	let handle = dev.open().unwrap();

	let t1 = thread::scoped(|| {
		println!("1 Opened device {:?}", handle.ptr());
		println!("ctrl {:?}", handle.ctrl_read(0xC0, 0x20, 0, 0, 64, 0));
		println!("Write {:?}", handle.write(0x02, libusb::LIBUSB_TRANSFER_TYPE_BULK, &[1,2,3], 0));
	});

	let t2 = thread::scoped(|| {
		println!("2 Opened device {:?}", handle.ptr());
		println!("ctrl {:?}", handle.ctrl_write(0x40, 0x81, 0, 0, &[1,2,3], 0));
		println!("Read {:?}", handle.read(0x81, libusb::LIBUSB_TRANSFER_TYPE_BULK, 64, 0));
	});

	t1.join();
	t2.join();

}
