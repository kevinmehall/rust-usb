use libusb::*;
use std::unstable::intrinsics;
use std::libc::{c_int, c_uint, c_void, size_t, uint8_t, uint16_t};
use std::vec;
use std::ptr::{to_unsafe_ptr, to_mut_unsafe_ptr};
use std::result::Result;
use std::iterator::IteratorUtil;
use std::task;
use std::comm::{PortOne, ChanOne, oneshot};
use std::cast::transmute;
use std::sys::size_of;


use std::unstable::sync::UnsafeAtomicRcBox;
use std::unstable::atomics::{AtomicInt, SeqCst};


pub struct ContextData {
	priv ctx: *mut libusb_context,
	priv open_device_count: AtomicInt
}

impl Drop for ContextData {
	fn drop(&self) {
		unsafe {
			assert!(self.open_device_count.load(SeqCst) == 0);
			// TODO: make sure backend thread is dead if the last device just closed
			libusb_exit(self.ctx);
		}
	}
}

pub struct Context {
	priv box: UnsafeAtomicRcBox<ContextData>
}

impl Context {
	pub fn new() -> Context {
		unsafe{
			let mut ctx: *mut libusb_context = intrinsics::init();
			let r = libusb_init(&mut ctx);

			Context{
				box: UnsafeAtomicRcBox::new(ContextData{
					ctx: ctx,
					open_device_count: AtomicInt::new(0)
				})
			}
		}
	}

	pub fn ptr(&self) -> *mut libusb_context {
		unsafe{
			(*self.box.get()).ctx
		}
	}

	pub fn setDebug(&self, level: int) {
		unsafe{
			libusb_set_debug(self.ptr(), level as c_int);
		}
	}

	pub fn listDevices(&self) -> ~[Device] {
		unsafe{
			let mut list: *mut *mut libusb_device = intrinsics::init();
			let num_devices = libusb_get_device_list(self.ptr(), &mut list);
			let r = vec::raw::mut_buf_as_slice(list, num_devices as uint, |l|{
				l.iter().transform(|i| Device{dev: *i, ctx: (*self).clone()}).collect()
			});

			libusb_free_device_list(list, 0);
			r
		}
	}

	pub fn find_by_vid_pid(&self, vid: uint, pid: uint) -> Option<Device> {
		self.listDevices().consume_iter().find_(|d| {
			let desc = d.descriptor();
			desc.idVendor as uint == vid && desc.idProduct as uint == pid
		})
	}

	priv fn device_opened(&self) {
		let count = unsafe { &mut (*self.box.get()).open_device_count };
		let old_count = count.fetch_add(1, SeqCst);

		if old_count == 0 {
			println("Starting task");

			let tbox = self.box.clone();

			do task::spawn_sched(task::SingleThreaded) {
				unsafe {
					let ctx = (*tbox.get()).ctx;
					let count = &(*tbox.get()).open_device_count;

					while (count.load(SeqCst) > 0) {
						println("Task looped");
						libusb_handle_events(ctx);
					}

					println("Task exited");
				}
			}
		}
	}

	priv fn device_closed(&self) {
		let count = unsafe { &mut (*self.box.get()).open_device_count };
		count.fetch_sub(1, SeqCst);
	}
}

extern fn rust_usb_callback(transfer: *mut libusb_transfer) {
	unsafe {
		println("Got callback");
		let chan: ~ChanOne<()> = transmute((*transfer).user_data);
		chan.send(());
	}
}

impl Clone for Context{
	fn clone(&self) -> Context{
		Context{box: self.box.clone()}
	}
}

pub struct Device {
	priv dev: *mut libusb_device,
	ctx: Context
}

impl Device {
	pub fn descriptor(&self) -> ~libusb_device_descriptor {
		unsafe{
			let mut d: ~libusb_device_descriptor = ~intrinsics::uninit();
			libusb_get_device_descriptor(self.dev, to_mut_unsafe_ptr(d));
			d
		}
	}

	pub fn bus(&self) -> int {
		unsafe {
			libusb_get_bus_number(self.dev) as int
		}
	}

	pub fn address(&self) -> int {
		unsafe {
			libusb_get_device_address(self.dev) as int
		}
	}

	pub fn open(&self) -> Result<DeviceHandle, int> {
		unsafe {
			let mut handle: *mut libusb_device_handle = intrinsics::uninit();
			let r = libusb_open(self.dev, &mut handle);
			if (r == 0){
				self.ctx.device_opened();
				Ok(DeviceHandle {
					box: UnsafeAtomicRcBox::new(DeviceHandleData {
						dev: handle,
						ctx: self.ctx.clone()
					})
				})
			}else{
				Err(r as int)
			}
		}
	}
}

impl Drop for Device {
	fn drop(&self) {
		unsafe {
			println(fmt!("Dropping device %i.%i at %x", self.bus(), self.address(), to_unsafe_ptr(self) as uint));
			libusb_unref_device(self.dev);
		}
	}
}


impl Clone for Device {
	fn clone(&self) -> Device {
		unsafe {
			println(fmt!("Cloning device %i.%i at %x", self.bus(), self.address(), to_unsafe_ptr(self) as uint));
			libusb_ref_device(self.dev);
		}
		Device{dev: self.dev, ctx: self.ctx.clone()}
	}
}

struct DeviceHandleData{
	priv dev: *mut libusb_device_handle,
	priv ctx: Context
}

impl Drop for DeviceHandleData {
	fn drop(&self) {
		unsafe {
			println(fmt!("Dropping DeviceHandleData %?", self));
			self.ctx.device_closed();
			libusb_close(self.dev);
		}
	}
}

pub struct DeviceHandle {
	priv box: UnsafeAtomicRcBox<DeviceHandleData>
}

impl DeviceHandle {
	pub fn ptr(&self) -> *mut libusb_device_handle {
		unsafe {
			(*self.box.get()).dev
		}
	}

	pub unsafe fn submit_transfer_sync(&self,
		endpoint: u8,
		transfer_type: libusb_transfer_type,
		length: uint,
		buffer: *mut u8) -> (libusb_transfer_status, uint) {

		let (port, chan): (PortOne<()>, ChanOne<()>) = oneshot();

		let mut t = libusb_alloc_transfer(0);
		(*t).dev_handle = self.ptr();
		(*t).endpoint = endpoint;
		(*t).transfer_type = transfer_type as u8;
		(*t).timeout = 0;
		(*t).length = length as i32;
		(*t).callback = rust_usb_callback;
		(*t).user_data = transmute(~chan);
		(*t).buffer = buffer;

		println(fmt!("Submitted %?", t));

		libusb_submit_transfer(t);
		port.recv();
		let r = ((*t).status, (*t).actual_length as uint);
		libusb_free_transfer(t);
		return r;
	}

	pub fn read(&self,
			endpoint: u8,
			transfer_type: libusb_transfer_type,
			size: uint
			) -> Result<~[u8], libusb_transfer_status> {
		let mut buf: ~[u8] = vec::from_elem(size, 0);
		unsafe {
			let ptr = vec::raw::to_mut_ptr(buf);
			let (status, actual_length) = self.submit_transfer_sync(
				endpoint, transfer_type, size, ptr);

			if status == LIBUSB_TRANSFER_COMPLETED {
				buf.truncate(actual_length);
				Ok(buf)
			} else {
				Err(status)
			}
		}
	}

	pub fn write(&self, endpoint: u8, transfer_type: libusb_transfer_type, buf: &[u8]) -> Result<(), libusb_transfer_status> {
		unsafe {
			let ptr = vec::raw::to_ptr(buf) as *mut u8;

			let (status, _) = self.submit_transfer_sync(
				endpoint, transfer_type, buf.len(), ptr);

			if status == LIBUSB_TRANSFER_COMPLETED {
				Ok(())
			} else {
				Err(status)
			}
		}
	}

	pub fn ctrl_read(&self, bmRequestType: u8, bRequest: u8,
		wValue:u16, wIndex: u16, length: uint) -> Result<~[u8], libusb_transfer_status> {

		let setup_length = size_of::<libusb_control_setup>();
		let total_length = setup_length + length as uint;
		let mut buf: ~[u8] = vec::from_elem(total_length, 0);

		unsafe{
			let ptr = fill_setup_buf(buf, bmRequestType, bRequest, wValue, wIndex, length);

			let (status, actual_length) = self.submit_transfer_sync(
				0, LIBUSB_TRANSFER_TYPE_CONTROL, total_length, ptr);

			if status == LIBUSB_TRANSFER_COMPLETED {
				Ok(buf.slice(setup_length, setup_length+actual_length).to_owned())
			} else {
				Err(status)
			}
		}
	}

	pub fn ctrl_write(&self, bmRequestType: u8, bRequest: u8,
		wValue:u16, wIndex: u16, buf: &[u8]) -> Result<(), libusb_transfer_status> {
		unsafe {
			let mut setup_buf = vec::from_elem(size_of::<libusb_control_setup>(), 0);
			fill_setup_buf(setup_buf, bmRequestType, bRequest, wValue, wIndex, buf.len());
			self.write(0, LIBUSB_TRANSFER_TYPE_CONTROL, setup_buf+buf)
		}
	}
}

unsafe fn fill_setup_buf(buf: &mut [u8], bmRequestType: u8,
	bRequest: u8, wValue:u16, wIndex: u16, length: uint) -> *mut u8 {
	let ptr = vec::raw::to_mut_ptr(buf);
	let setup = ptr as *mut libusb_control_setup;

	// TODO: these are always little-endian
	(*setup).bmRequestType = bmRequestType;
	(*setup).bRequest = bRequest;
	(*setup).wValue = wValue;
	(*setup).wIndex = wIndex;
	(*setup).wLength = length as u16;

	return ptr;
}

impl Clone for DeviceHandle {
	fn clone(&self) -> DeviceHandle {
		println(fmt!("Cloning devicehandle %?", self.ptr()));
		DeviceHandle{box: self.box.clone()}
	}
}

