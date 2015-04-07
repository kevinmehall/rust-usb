#![ feature(libc, core, collections, unsafe_destructor) ]
#![ allow(non_snake_case) ]

extern crate libc;

use libusb::*;
use libc::c_int;
use std::intrinsics;
use std::slice;
use std::iter::repeat;
use std::result::Result;
use std::mem::transmute;
use std::mem::size_of;
use std::vec::Vec;
use std::cell::UnsafeCell;

pub mod libusb;

pub struct Context(*mut libusb_context);
unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
	pub fn new() -> Context {
		unsafe{
			let mut ctx: *mut libusb_context = intrinsics::init();
			let r = libusb_init(&mut ctx);
			assert!(r == 0);
			Context(ctx)
		}
	}

	pub fn ptr(&self) -> *mut libusb_context {
		self.0
	}

	pub fn set_debug(&self, level: u32) {
		unsafe{
			libusb_set_debug(self.ptr(), level as c_int);
		}
	}

	pub fn list_devices(&self) -> Vec<Device> {
		unsafe{
			let mut list: *mut *mut libusb_device = intrinsics::init();
			let num_devices = libusb_get_device_list(self.ptr(), &mut list);
			let l = slice::from_raw_parts_mut(list, num_devices as usize);
			let devices = l.iter().map(|i| Device { dev: *i, ctx: self }).collect();
			libusb_free_device_list(list, 0);
			devices
		}
	}

	pub fn find_by_vid_pid(&self, vid: u16, pid: u16) -> Option<Device> {
		self.list_devices().into_iter().find(|d| {
			let desc = d.descriptor();
			desc.idVendor == vid && desc.idProduct == pid
		})
	}
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe { libusb_exit(self.0); }
	}
}

pub struct Device<'c> {
	dev: *mut libusb_device,
	ctx: &'c Context,
}

impl<'c> Device<'c> {
	pub fn descriptor(&self) -> libusb_device_descriptor {
		unsafe{
			let mut d: libusb_device_descriptor = intrinsics::uninit();
			libusb_get_device_descriptor(self.dev, &mut d as *mut libusb_device_descriptor);
			d
		}
	}

	pub fn bus(&self) -> u8 {
		unsafe {
			libusb_get_bus_number(self.dev)
		}
	}

	pub fn address(&self) -> u8 {
		unsafe {
			libusb_get_device_address(self.dev)
		}
	}

	pub fn open(&self) -> Result<DeviceHandle, c_int> {
		unsafe {
			let mut handle: *mut libusb_device_handle = intrinsics::uninit();
			let r = libusb_open(self.dev, &mut handle);
			if r == 0 {
				Ok(DeviceHandle {
					dev: handle,
					ctx: self.ctx
				})
			}else{
				Err(r)
			}
		}
	}
}

#[unsafe_destructor]
impl<'c> Drop for Device<'c> {
	fn drop(&mut self) {
		unsafe {
			libusb_unref_device(self.dev);
		}
	}
}


impl<'c> Clone for Device<'c> {
	fn clone(&self) -> Device<'c> {
		unsafe {
			libusb_ref_device(self.dev);
		}
		Device{ dev: self.dev, ctx: self.ctx }
	}
}

pub struct DeviceHandle<'c> {
	dev: *mut libusb_device_handle,
	ctx: &'c Context
}
unsafe impl<'c> Sync for DeviceHandle<'c> {}

impl<'c> DeviceHandle<'c> {
	pub fn ptr(&self) -> *mut libusb_device_handle {
		self.dev
	}

	pub fn claim_interface(&self, iface_num: u16) {
		unsafe {
			libusb_claim_interface(self.ptr(), iface_num as c_int);
		}
	}

	pub unsafe fn submit_transfer_sync(&self,
		endpoint: u8,
		transfer_type: libusb_transfer_type,
		length: usize,
		buffer: *mut u8,
		timeout: u32) -> (libusb_transfer_status, usize) {

		let completed: UnsafeCell<c_int> = UnsafeCell::new(0);

		extern fn callback(transfer: *mut libusb_transfer) {
			unsafe {
				let completed: &UnsafeCell<c_int> = transmute((*transfer).user_data);
				*completed.get() = 1;
			}
		}

		let t = libusb_alloc_transfer(0);
		(*t).dev_handle = self.ptr();
		(*t).endpoint = endpoint;
		(*t).transfer_type = transfer_type as u8;
		(*t).timeout = timeout;
		(*t).length = length as c_int;
		(*t).callback = callback;
		(*t).user_data = transmute(&completed);
		(*t).buffer = buffer;

		libusb_submit_transfer(t);

		while *completed.get() == 0{
			libusb_handle_events_completed(self.ctx.ptr(), completed.get());
		}

		let r = ((*t).get_status(), (*t).actual_length as usize);
		libusb_free_transfer(t);
		return r;
	}

	pub fn read(&self,
			endpoint: u8,
			transfer_type: libusb_transfer_type,
			size: usize,
			timeout: u32
			) -> Result<Vec<u8>, libusb_transfer_status> {
		let mut buf: Vec<u8> = repeat(0u8).take(size).collect();
		unsafe {
			let ptr = buf.as_mut_ptr();
			let (status, actual_length) = self.submit_transfer_sync(
				endpoint, transfer_type, size, ptr, timeout);

			if status == LIBUSB_TRANSFER_COMPLETED {
				buf.truncate(actual_length);
				Ok(buf)
			} else {
				Err(status)
			}
		}
	}

	pub fn write(&self,
			endpoint: u8,
			transfer_type: libusb_transfer_type,
			buf: &[u8],
			timeout: u32
			) -> Result<(), libusb_transfer_status> {
		unsafe {
			let ptr = buf.as_ptr() as *mut u8;

			let (status, _) = self.submit_transfer_sync(
				endpoint, transfer_type, buf.len(), ptr, timeout);

			if status == LIBUSB_TRANSFER_COMPLETED {
				Ok(())
			} else {
				Err(status)
			}
		}
	}

	pub fn ctrl_read(&self, bmRequestType: u8, bRequest: u8,
		wValue:u16, wIndex: u16, length: usize, timeout: u32) -> Result<Vec<u8>, libusb_transfer_status> {

		let setup_length = size_of::<libusb_control_setup>();
		let total_length = setup_length + length;
		let mut buf: Vec<u8> = repeat(0u8).take(total_length).collect();
		let ptr = fill_setup_buf(&mut buf, bmRequestType, bRequest, wValue, wIndex, length);

		unsafe{
			let (status, actual_length) = self.submit_transfer_sync(
				0, LIBUSB_TRANSFER_TYPE_CONTROL, total_length, ptr, timeout);

			if status == LIBUSB_TRANSFER_COMPLETED {
				Ok(buf[setup_length..setup_length+actual_length].to_vec())
			} else {
				Err(status)
			}
		}
	}

	pub fn ctrl_write(&self, bmRequestType: u8, bRequest: u8,
		wValue:u16, wIndex: u16, buf: &[u8], timeout: u32) -> Result<(), libusb_transfer_status> {
		let mut setup_buf: Vec<_> = repeat(0u8).take(size_of::<libusb_control_setup>()).collect();
		fill_setup_buf(&mut setup_buf, bmRequestType, bRequest, wValue, wIndex, buf.len());
		setup_buf.push_all(buf);
		self.write(0, LIBUSB_TRANSFER_TYPE_CONTROL, &setup_buf, timeout)
	}
}

#[unsafe_destructor]
impl<'c> Drop for DeviceHandle<'c> {
	fn drop(&mut self) {
		unsafe {
			libusb_close(self.dev);
		}
	}
}

fn fill_setup_buf(buf: &mut [u8], bmRequestType: u8,
	bRequest: u8, wValue:u16, wIndex: u16, length: usize) -> *mut u8 {
	let ptr = buf.as_mut_ptr();
	let setup = ptr as *mut libusb_control_setup;

	assert!(buf.len() >= 8);
	assert!(length <= (std::u16::MAX as usize));

	unsafe {
		(*setup).bmRequestType = bmRequestType;
		(*setup).bRequest = bRequest;
		(*setup).wValue = wValue.to_le();
		(*setup).wIndex = wIndex.to_le();
		(*setup).wLength = (length as u16).to_le();
	}

	return ptr;
}
