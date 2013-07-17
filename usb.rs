use libusb::*;
use std::unstable::intrinsics;
use std::libc::{c_int, c_uint, c_void, size_t, uint8_t, uint16_t};
use std::vec;
use std::ptr::{to_unsafe_ptr, to_mut_unsafe_ptr};
use std::result::Result;
use std::iterator::IteratorUtil;
use std::task;

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
}

impl Clone for DeviceHandle {
	fn clone(&self) -> DeviceHandle {
		println(fmt!("Cloning devicehandle %?", self.ptr()));
		DeviceHandle{box: self.box.clone()}
	}
}

