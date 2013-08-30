use libusb::*;
use std::unstable::intrinsics;
use std::libc::{c_int, c_void, size_t, malloc, free};
use std::vec;
use std::ptr::{to_mut_unsafe_ptr};
use std::result::Result;
use std::task;
use std::comm::{PortOne, ChanOne, SharedChan, stream, oneshot};
use std::cast::transmute;
use std::sys::size_of;

use std::unstable::sync::UnsafeArc;
use std::unstable::atomics::{AtomicInt, SeqCst};


pub struct ContextData {
	priv ctx: *mut libusb_context,
	priv open_device_count: AtomicInt
}

impl Drop for ContextData {
	#[fixed_stack_segment]
	#[inline(never)]
	fn drop(&self) {
		unsafe {
			assert!(self.open_device_count.load(SeqCst) == 0);
			// TODO: make sure backend thread is dead if the last device just closed
			libusb_exit(self.ctx);
		}
	}
}

pub struct Context {
	priv box: UnsafeArc<ContextData>
}

impl Context {
	#[fixed_stack_segment]
	#[inline(never)]
	pub fn new() -> Context {
		unsafe{
			let mut ctx: *mut libusb_context = intrinsics::init();
			let r = libusb_init(&mut ctx);

			Context{
				box: UnsafeArc::new(ContextData{
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

	#[fixed_stack_segment]
	#[inline(never)]
	pub fn setDebug(&self, level: int) {
		unsafe{
			libusb_set_debug(self.ptr(), level as c_int);
		}
	}

	#[fixed_stack_segment]
	#[inline(never)]
	pub fn listDevices(&self) -> ~[Device] {
		unsafe{
			let mut list: *mut *mut libusb_device = intrinsics::init();
			let num_devices = libusb_get_device_list(self.ptr(), &mut list);
			let r = vec::raw::mut_buf_as_slice(list, num_devices as uint, |l|{
				l.iter().map(|i| Device{dev: *i, ctx: (*self).clone()}).collect()
			});

			libusb_free_device_list(list, 0);
			r
		}
	}

	pub fn find_by_vid_pid(&self, vid: uint, pid: uint) -> Option<Device> {
		self.listDevices().move_iter().find(|d| {
			let desc = d.descriptor();
			desc.idVendor as uint == vid && desc.idProduct as uint == pid
		})
	}

	fn device_opened(&self) {
		let count = unsafe { &mut (*self.box.get()).open_device_count };
		let old_count = count.fetch_add(1, SeqCst);

		if old_count == 0 {
			let box = self.box.clone();

			#[fixed_stack_segment]
			#[inline(never)]
			fn threadfn(tbox: &UnsafeArc<ContextData>) {
				unsafe {
					let ctx = (*tbox.get()).ctx;
					let count = &(*tbox.get()).open_device_count;

					while (count.load(SeqCst) > 0) {
						libusb_handle_events(ctx);
					}
				}
			}

			do task::spawn_sched(task::SingleThreaded) {
				threadfn(&box);
			}
		}
	}

	fn device_closed(&self) {
		let count = unsafe { &mut (*self.box.get()).open_device_count };
		count.fetch_sub(1, SeqCst);
	}
}

extern fn rust_usb_callback(transfer: *mut libusb_transfer) {
	unsafe {
		let chan: ~ChanOne<()> = transmute((*transfer).user_data);
		chan.send(());
	}
}

struct TH {
	t: *mut libusb_transfer,
	c: SharedChan<*mut libusb_transfer>,
}

impl Drop for TH{
	#[fixed_stack_segment]
	#[inline(never)]
	fn drop(&self) {
		unsafe {
			free((*self.t).buffer as *c_void);
			libusb_free_transfer(self.t);
		}
	}
}

extern fn rust_usb_stream_callback(transfer: *mut libusb_transfer) {
	unsafe {
		let chan: &SharedChan<*mut libusb_transfer> = transmute((*transfer).user_data);
		chan.send(transfer);
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
	#[fixed_stack_segment]
	#[inline(never)]
	pub fn descriptor(&self) -> ~libusb_device_descriptor {
		unsafe{
			let mut d: ~libusb_device_descriptor = ~intrinsics::uninit();
			libusb_get_device_descriptor(self.dev, to_mut_unsafe_ptr(d));
			d
		}
	}

	#[fixed_stack_segment]
	#[inline(never)]
	pub fn bus(&self) -> int {
		unsafe {
			libusb_get_bus_number(self.dev) as int
		}
	}

	#[fixed_stack_segment]
	#[inline(never)]
	pub fn address(&self) -> int {
		unsafe {
			libusb_get_device_address(self.dev) as int
		}
	}

	#[fixed_stack_segment]
	#[inline(never)]
	pub fn open(&self) -> Result<DeviceHandle, int> {
		unsafe {
			let mut handle: *mut libusb_device_handle = intrinsics::uninit();
			let r = libusb_open(self.dev, &mut handle);
			if (r == 0){
				self.ctx.device_opened();
				Ok(DeviceHandle {
					box: UnsafeArc::new(DeviceHandleData {
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
	#[fixed_stack_segment]
	#[inline(never)]
	fn drop(&self) {
		unsafe {
			libusb_unref_device(self.dev);
		}
	}
}


impl Clone for Device {
	#[fixed_stack_segment]
	#[inline(never)]
	fn clone(&self) -> Device {
		unsafe {
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
	#[fixed_stack_segment]
	#[inline(never)]
	fn drop(&self) {
		unsafe {
			self.ctx.device_closed();
			libusb_close(self.dev);
		}
	}
}

pub struct DeviceHandle {
	priv box: UnsafeArc<DeviceHandleData>
}

impl DeviceHandle {
	pub fn ptr(&self) -> *mut libusb_device_handle {
		unsafe {
			(*self.box.get()).dev
		}
	}

	#[fixed_stack_segment]
	#[inline(never)]
	pub unsafe fn submit_transfer_sync(&self,
		endpoint: u8,
		transfer_type: libusb_transfer_type,
		length: uint,
		buffer: *mut u8) -> (libusb_transfer_status, uint) {

		let (port, chan): (PortOne<()>, ChanOne<()>) = oneshot();

		let t = libusb_alloc_transfer(0);
		(*t).dev_handle = self.ptr();
		(*t).endpoint = endpoint;
		(*t).transfer_type = transfer_type as u8;
		(*t).timeout = 0;
		(*t).length = length as c_int;
		(*t).callback = rust_usb_callback;
		(*t).user_data = transmute(~chan);
		(*t).buffer = buffer;

		libusb_submit_transfer(t);
		port.recv();
		let r = ((*t).get_status(), (*t).actual_length as uint);
		libusb_free_transfer(t);
		return r;
	}

	pub fn read(&self,
			endpoint: u8,
			transfer_type: libusb_transfer_type,
			size: uint
			) -> Result<~[u8], libusb_transfer_status> {
		let mut buf: ~[u8] = vec::from_elem(size, 0u8);
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
		let mut buf: ~[u8] = vec::from_elem(total_length, 0u8);

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
			let mut setup_buf = vec::from_elem(size_of::<libusb_control_setup>(), 0u8);
			fill_setup_buf(setup_buf, bmRequestType, bRequest, wValue, wIndex, buf.len());
			self.write(0, LIBUSB_TRANSFER_TYPE_CONTROL, setup_buf+buf)
		}
	}

	#[fixed_stack_segment]
	#[inline(never)]
	unsafe fn stream_transfers(&self, endpoint: u8,
			transfer_type: libusb_transfer_type, size: uint,
			num_transfers: uint) -> (Port<*mut libusb_transfer>, ~[TH]) {
		
		let (port, chan) = stream::<*mut libusb_transfer>();
		let sc = SharedChan::new(chan);

		let transfers = do vec::from_fn(num_transfers) |_| { TH {
			t: libusb_alloc_transfer(0),
			c: sc.clone(),
		}};

		for th in transfers.iter() {
			(*th.t).dev_handle = self.ptr();
			(*th.t).endpoint = endpoint;
			(*th.t).transfer_type = transfer_type as u8;
			(*th.t).timeout = 0;
			(*th.t).length = size as i32;
			(*th.t).callback = rust_usb_stream_callback;
			(*th.t).buffer = malloc(size as size_t) as *mut u8;
			(*th.t).user_data = transmute(&th.c);
		}

		return (port, transfers);
	}

	#[fixed_stack_segment]
	#[inline(never)]
	pub fn read_stream(&self, endpoint: u8,
			transfer_type: libusb_transfer_type,
			size: uint, mut num_transfers: uint, cb: &fn(Result<&[u8], libusb_transfer_status>) -> bool) {

		unsafe {
			let mut running = true;
			let (port, transfers) = self.stream_transfers(
				endpoint, transfer_type, size, num_transfers);

			for th in transfers.iter() {
				libusb_submit_transfer(th.t);
			}

			while (num_transfers > 0) {
				let transfer: *mut libusb_transfer = port.recv();

				if ((*transfer).get_status() == LIBUSB_TRANSFER_COMPLETED) {
					do vec::raw::buf_as_slice((*transfer).buffer as *u8, size) |b| {
						running &= cb(Ok(b))
					}
				} else {
					running = false;
					running &= cb(Err((*transfer).get_status()));
				}

				if (running) {
					let r = libusb_submit_transfer(transfer);
					assert!(r == 0);
				} else {
					num_transfers -= 1;
				}
			}
		}
	}

	#[fixed_stack_segment]
	#[inline(never)]
	pub fn write_stream(&self, endpoint: u8,
			transfer_type: libusb_transfer_type,
			size: uint, num_transfers: uint, cb: &fn(Result<(&mut[u8]), libusb_transfer_status>) -> bool) {

		unsafe {
			let mut running = true;
			let mut running_transfers = 0;
			let (port, transfers) = self.stream_transfers(
				endpoint, transfer_type, size, num_transfers);

			for th in transfers.iter() {
				do vec::raw::mut_buf_as_slice((*th.t).buffer, size) |b| {
					running &= cb(Ok(b));
				}
				if running {
					libusb_submit_transfer(th.t);
					running_transfers += 1;
				} else {
					break;
				}
			}

			while (running_transfers > 0) {
				let transfer: *mut libusb_transfer = port.recv();

				if ((*transfer).get_status() == LIBUSB_TRANSFER_COMPLETED) {
					do vec::raw::mut_buf_as_slice((*transfer).buffer, size) |b| {
						running &= cb(Ok(b))
					}
				} else {
					running = false;
					running &= cb(Err((*transfer).get_status()));
				}

				if (running) {
					let r = libusb_submit_transfer(transfer);
					assert!(r == 0);
				} else {
					running_transfers -= 1;
				}
			}
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
		DeviceHandle{box: self.box.clone()}
	}
}
