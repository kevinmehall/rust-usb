#![allow(non_camel_case_types)]

use libc::{c_int, c_uint, c_void, size_t, uint8_t, uint16_t};
use std::num;
use std::fmt::Show;

pub struct libusb_context;
pub struct libusb_device;
pub struct libusb_device_handle;

#[link (name="usb-1.0")]

extern{
	pub fn libusb_init(ctx: *mut *mut libusb_context) -> c_int;
	pub fn libusb_exit(ctx: *mut libusb_context);
	pub fn libusb_set_debug(ctx: *mut libusb_context, level: c_int);
	pub fn libusb_handle_events(ctx: *mut libusb_context) -> c_int;

	pub fn libusb_get_device_list(ctx: *mut libusb_context, list: *mut *mut *mut libusb_device) -> size_t;
	pub fn libusb_free_device_list(list: *mut *mut libusb_device, unref_devices: c_int);

	pub fn libusb_ref_device(dev: *mut libusb_device) -> *mut libusb_device;
	pub fn libusb_unref_device(dev: *mut libusb_device);
	pub fn libusb_get_configuration(dev: *mut libusb_device_handle, config: *mut c_int) -> c_int;
	pub fn libusb_get_device_descriptor(dev: *mut libusb_device, desc: *mut libusb_device_descriptor) -> c_int;
	pub fn libusb_get_active_config_descriptor(dev: *mut libusb_device, config: *mut *mut libusb_config_descriptor) -> c_int;
	pub fn libusb_get_config_descriptor(dev: *mut libusb_device, config_index: uint8_t, config: *mut *mut libusb_config_descriptor) -> c_int;
	pub fn libusb_get_config_descriptor_by_value(dev: *mut libusb_device, bConfigurationValue: uint8_t, config: *mut *mut libusb_config_descriptor) -> c_int;
	pub fn libusb_free_config_descriptor(config: *mut libusb_config_descriptor);

	pub fn libusb_get_bus_number(dev: *mut libusb_device) -> uint8_t;
	pub fn libusb_get_port_number(dev: *mut libusb_device) -> uint8_t;
	pub fn libusb_get_device_address(dev: *mut libusb_device) -> uint8_t;
	pub fn libusb_get_device_speed(dev: *mut libusb_device) -> c_int;
	pub fn libusb_get_max_packet_size(dev: *mut libusb_device, endpoint: uint8_t) -> c_int;
	pub fn libusb_get_max_iso_packet_size(dev: *mut libusb_device, endpoint: uint8_t) -> c_int;

	pub fn libusb_open(dev: *mut libusb_device,  handle: *mut *mut libusb_device_handle) -> c_int;
	pub fn libusb_close(dev_handle: *mut libusb_device_handle);
	pub fn libusb_get_device(dev_handle: *mut libusb_device_handle) -> *mut libusb_device;

	pub fn libusb_set_configuration(dev: *mut libusb_device_handle, configuration: c_int) -> c_int;
	pub fn libusb_claim_interface(dev: *mut libusb_device_handle, interface_number: c_int) -> c_int;
	pub fn libusb_release_interface(dev: *mut libusb_device_handle, interface_number: c_int) -> c_int;

	pub fn libusb_set_interface_alt_setting(dev: *mut libusb_device_handle, interface_number: c_int, alternate_setting: c_int) -> c_int;
	pub fn libusb_clear_halt(dev: *mut libusb_device_handle, endpoint: uint8_t) -> c_int;
	pub fn libusb_reset_device(dev: *mut libusb_device_handle) -> c_int;

	pub fn libusb_kernel_driver_active(dev: *mut libusb_device_handle, interface_number: c_int) -> c_int;
	pub fn libusb_detach_kernel_driver(dev: *mut libusb_device_handle, interface_number: c_int) -> c_int;
	pub fn libusb_attach_kernel_driver(dev: *mut libusb_device_handle, interface_number: c_int) -> c_int;

	pub fn libusb_alloc_transfer(iso_packets: c_int) -> *mut libusb_transfer;
	pub fn libusb_submit_transfer(transfer: *mut libusb_transfer) -> c_int;
	pub fn libusb_cancel_transfer(transfer: *mut libusb_transfer) -> c_int;
	pub fn libusb_free_transfer(transfer: *mut libusb_transfer);
}

/**
 * Device and/or Interface Class codes */
pub enum libusb_class_code {
	/** In the context of a \ref libusb_device_descriptor "device descriptor",
	 * this bDeviceClass value indicates that each interface specifies its
	 * own class information and all interfaces operate independently.
	 */
	LIBUSB_CLASS_PER_INTERFACE = 0,

	/** Audio class */
	LIBUSB_CLASS_AUDIO = 1,

	/** Communications class */
	LIBUSB_CLASS_COMM = 2,

	/** Human Interface Device class */
	LIBUSB_CLASS_HID = 3,

	/** Physical */
	LIBUSB_CLASS_PHYSICAL = 5,

	/** Printer class */
	LIBUSB_CLASS_PRINTER = 7,

	/** Image class */
	LIBUSB_CLASS_IMAGE = 6,

	/** Mass storage class */
	LIBUSB_CLASS_MASS_STORAGE = 8,

	/** Hub class */
	LIBUSB_CLASS_HUB = 9,

	/** Data class */
	LIBUSB_CLASS_DATA = 10,

	/** Smart Card */
	LIBUSB_CLASS_SMART_CARD = 0x0b,

	/** Content Security */
	LIBUSB_CLASS_CONTENT_SECURITY = 0x0d,

	/** Video */
	LIBUSB_CLASS_VIDEO = 0x0e,

	/** Personal Healthcare */
	LIBUSB_CLASS_PERSONAL_HEALTHCARE = 0x0f,

	/** Diagnostic Device */
	LIBUSB_CLASS_DIAGNOSTIC_DEVICE = 0xdc,

	/** Wireless class */
	LIBUSB_CLASS_WIRELESS = 0xe0,

	/** Application class */
	LIBUSB_CLASS_APPLICATION = 0xfe,

	/** Class is vendor-specific */
	LIBUSB_CLASS_VENDOR_SPEC = 0xff
}

/**
 * Descriptor types as defined by the USB specification. */
pub enum libusb_descriptor_type {
	/** Device descriptor. See libusb_device_descriptor. */
	LIBUSB_DT_DEVICE = 0x01,

	/** Configuration descriptor. See libusb_config_descriptor. */
	LIBUSB_DT_CONFIG = 0x02,

	/** String descriptor */
	LIBUSB_DT_STRING = 0x03,

	/** Interface descriptor. See libusb_interface_descriptor. */
	LIBUSB_DT_INTERFACE = 0x04,

	/** Endpodescriptor: c_int. See libusb_endpoint_descriptor. */
	LIBUSB_DT_ENDPOINT = 0x05,

	/** BOS descriptor */
	LIBUSB_DT_BOS = 0x0f,

	/** Device Capability descriptor */
	LIBUSB_DT_DEVICE_CAPABILITY = 0x10,

	/** HID descriptor */
	LIBUSB_DT_HID = 0x21,

	/** HID report descriptor */
	LIBUSB_DT_REPORT = 0x22,

	/** Physical descriptor */
	LIBUSB_DT_PHYSICAL = 0x23,

	/** Hub descriptor */
	LIBUSB_DT_HUB = 0x29,

	/** SuperSpeed Hub descriptor */
	LIBUSB_DT_SUPERSPEED_HUB = 0x2a,

	/** SuperSpeed EndpoCompanion: c_int descriptor */
	LIBUSB_DT_SS_ENDPOINT_COMPANION = 0x30
}

pub enum libusb_endpoint_direction {
	/** In: device-to-host */
	LIBUSB_ENDPOINT_IN = 0x80,

	/** Out: host-to-device */
	LIBUSB_ENDPOINT_OUT = 0x00
}

pub enum libusb_transfer_type {
	/** Control endpoint */
	LIBUSB_TRANSFER_TYPE_CONTROL = 0,

	/** Isochronous endpoint */
	LIBUSB_TRANSFER_TYPE_ISOCHRONOUS = 1,

	/** Bulk endpoint */
	LIBUSB_TRANSFER_TYPE_BULK = 2,

	/** Interrupt endpoint */
	LIBUSB_TRANSFER_TYPE_INTERRUPT = 3
}

/**
 * A structure representing the standard USB device descriptor. This
 * descriptor is documented in section 9.6.1 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
pub struct libusb_device_descriptor {
	/** Size of this descriptor (in bytes) */
	pub bLength: uint8_t,

	/** Descriptor type. Will have value
	 * \ref libusb_descriptor_type::LIBUSB_DT_DEVICE LIBUSB_DT_DEVICE in this
	 * context. */
	pub bDescriptorType: uint8_t,

	/** USB specification release number in binary-coded decimal. A value of
	 * 0x0200 indicates USB 2.0, 0x0110 indicates USB 1.1, etc. */
	pub bcdUSB: uint16_t,

	/** USB-IF class code for the device. See \ref libusb_class_code. */
	pub bDeviceClass: uint8_t,

	/** USB-IF subclass code for the device, qualified by the bDeviceClass
	 * value */
	pub bDeviceSubClass: uint8_t,

	/** USB-IF protocol code for the device, qualified by the bDeviceClass and
	 * bDeviceSubClass values */
	pub bDeviceProtocol: uint8_t,

	/** Maximum packet size for endpo0: c_int */
	pub bMaxPacketSize0: uint8_t,

	/** USB-IF vendor ID */
	pub idVendor: uint16_t,

	/** USB-IF product ID */
	pub idProduct: uint16_t,

	/** Device release number in binary-coded decimal */
	pub bcdDevice: uint16_t,

	/** Index of string descriptor describing manufacturer */
	pub iManufacturer: uint8_t,

	/** Index of string descriptor describing product */
	pub iProduct: uint8_t,

	/** Index of string descriptor containing device serial number */
	pub iSerialNumber: uint8_t,

	/** Number of possible configurations */
	pub bNumConfigurations: uint8_t,
}


/**
 * A structure representing the standard USB endpodescriptor: c_int. This
 * descriptor is documented in section 9.6.6 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
pub struct libusb_endpoint_descriptor {
	/** Size of this descriptor (in bytes) */
	pub bLength: uint8_t,

	/** Descriptor type. Will have value
	 * \ref libusb_descriptor_type::LIBUSB_DT_ENDPOLIBUSB_DT_ENDPOINT: c_int in
	 * this context. */
	pub bDescriptorType: uint8_t,

	/** The address of the endpodescribed: c_int by this descriptor. Bits 0:3 are
	 * the endponumber: c_int. Bits 4:6 are reserved. Bit 7 indicates direction,
	 * see \ref libusb_endpoint_direction.
	 */
	pub bEndpointAddress: uint8_t,

	/** Attributes which apply to the endpowhen: c_int it is configured using
	 * the bConfigurationValue. Bits 0:1 determine the transfer type and
	 * correspond to \ref libusb_transfer_type. Bits 2:3 are only used for
	 * isochronous endpoints and correspond to \ref libusb_iso_sync_type.
	 * Bits 4:5 are also only used for isochronous endpoints and correspond to
	 * \ref libusb_iso_usage_type. Bits 6:7 are reserved.
	 */
	pub bmAttributes: uint8_t,

	/** Maximum packet size this endpois: c_int capable of sending/receiving. */
	pub wMaxPacketSize: uint16_t,

	/** Interval for polling endpofor: c_int data transfers. */
	pub bInterval: uint8_t,

	/** For audio devices only: the rate at which synchronization feedback
	 * is provided. */
	pub bRefresh: uint8_t,

	/** For audio devices only: the address if the synch endpoint */
	pub bSynchAddress: uint8_t,

	/** Extra descriptors. If libusbx encounters unknown endpodescriptors: c_int,
	 * it will store them here, should you wish to parse them. */
	pub extra: *const uint8_t,

	/** Length of the extra descriptors, in bytes. */
	pub extra_length: int,
}

/**
 * A structure representing the standard USB interface descriptor. This
 * descriptor is documented in section 9.6.5 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
pub struct libusb_interface_descriptor {
	/** Size of this descriptor (in bytes) */
	pub bLength: uint8_t,

	/** Descriptor type. Will have value
	 * \ref libusb_descriptor_type::LIBUSB_DT_INTERFACE LIBUSB_DT_INTERFACE
	 * in this context. */
	pub bDescriptorType: uint8_t,

	/** Number of this interface */
	pub bInterfaceNumber: uint8_t,

	/** Value used to select this alternate setting for this interface */
	pub bAlternateSetting: uint8_t,

	/** Number of endpoints used by this interface (excluding the control
	 * endpoint). */
	pub bNumEndpoints: uint8_t,

	/** USB-IF class code for this interface. See \ref libusb_class_code. */
	pub bInterfaceClass: uint8_t,

	/** USB-IF subclass code for this interface, qualified by the
	 * bInterfaceClass value */
	pub bInterfaceSubClass: uint8_t,

	/** USB-IF protocol code for this interface, qualified by the
	 * bInterfaceClass and bInterfaceSubClass values */
	pub bInterfaceProtocol: uint8_t,

	/** Index of string descriptor describing this interface */
	pub iInterface: uint8_t,

	/** Array of endpodescriptors: c_int. This length of this array is determined
	 * by the bNumEndpoints field. */
	pub endpoint: *const libusb_endpoint_descriptor,

	/** Extra descriptors. If libusbx encounters unknown interface descriptors,
	 * it will store them here, should you wish to parse them. */
	pub extra: *const uint8_t,

	/** Length of the extra descriptors, in bytes. */
	pub extra_length: c_int,
}

/**
 * A collection of alternate settings for a particular USB interface.
 */
pub struct libusb_interface {
	/** Array of interface descriptors. The length of this array is determined
	 * by the num_altsetting field. */
	pub altsetting: libusb_interface_descriptor,

	/** The number of alternate settings that belong to this interface */
	pub num_altsetting: c_int,
}

/**
 * A structure representing the standard USB configuration descriptor. This
 * descriptor is documented in section 9.6.3 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
pub struct libusb_config_descriptor {
	/** Size of this descriptor (in bytes) */
	pub bLength: uint8_t,

	/** Descriptor type. Will have value
	 * \ref libusb_descriptor_type::LIBUSB_DT_CONFIG LIBUSB_DT_CONFIG
	 * in this context. */
	pub bDescriptorType: uint8_t,

	/** Total length of data returned for this configuration */
	pub wTotalLength: uint16_t,

	/** Number of interfaces supported by this configuration */
	pub bNumInterfaces: uint8_t,

	/** Identifier value for this configuration */
	pub bConfigurationValue: uint8_t,

	/** Index of string descriptor describing this configuration */
	pub iConfiguration: uint8_t,

	/** Configuration characteristics */
	pub bmAttributes: uint8_t,

	/** Maximum power consumption of the USB device from this bus in this
	 * configuration when the device is fully opreation. Expressed in units
	 * of 2 mA. */
	pub MaxPower: uint8_t,

	/** Array of interfaces supported by this configuration. The length of
	 * this array is determined by the bNumInterfaces field. */
	pub interface: *const libusb_interface,

	/** Extra descriptors. If libusbx encounters unknown configuration
	 * descriptors, it will store them here, should you wish to parse them. */
	pub extra: *const uint8_t,

	/** Length of the extra descriptors, in bytes. */
	pub extra_length: int,
}

/**
 * Error codes. Most libusbx functions return 0 on success or one of these
 * codes on failure.
 * You can call libusb_error_name() to retrieve a string representation of an
 * error code or libusb_strerror() to get an end-user suitable description of
 * an error code.
 */
 #[deriving(Show)]
pub enum libusb_error {
	/** Success (no error) */
	LIBUSB_SUCCESS = 0,

	/** Input/output error */
	LIBUSB_ERROR_IO = -1,

	/** Invalid parameter */
	LIBUSB_ERROR_INVALID_PARAM = -2,

	/** Access denied (insufficient permissions) */
	LIBUSB_ERROR_ACCESS = -3,

	/** No such device (it may have been disconnected) */
	LIBUSB_ERROR_NO_DEVICE = -4,

	/** Entity not found */
	LIBUSB_ERROR_NOT_FOUND = -5,

	/** Resource busy */
	LIBUSB_ERROR_BUSY = -6,

	/** Operation timed out */
	LIBUSB_ERROR_TIMEOUT = -7,

	/** Overflow */
	LIBUSB_ERROR_OVERFLOW = -8,

	/** Pipe error */
	LIBUSB_ERROR_PIPE = -9,

	/** System call interrupted (perhaps due to signal) */
	LIBUSB_ERROR_INTERRUPTED = -10,

	/** Insufficient memory */
	LIBUSB_ERROR_NO_MEM = -11,

	/** Operation not supported or unimplemented on this platform */
	LIBUSB_ERROR_NOT_SUPPORTED = -12,

	/* NB: Remember to update LIBUSB_ERROR_COUNT below as well as the
	   message strings in strerror.c when adding new error codes here. */

	/** Other error */
	LIBUSB_ERROR_OTHER = -99,
}

/** \ingroup asyncio
 * Transfer status codes */
#[deriving(FromPrimitive,Show,PartialEq)]
pub enum libusb_transfer_status {
	/** Transfer completed without error. Note that this does not indicate
	 * that the entire amount of requested data was transferred. */
	LIBUSB_TRANSFER_COMPLETED = 0,

	/** Transfer failed */
	LIBUSB_TRANSFER_ERROR,

	/** Transfer timed out */
	LIBUSB_TRANSFER_TIMED_OUT,

	/** Transfer was cancelled */
	LIBUSB_TRANSFER_CANCELLED,

	/** For bulk/interrupt endpoints: halt condition detected (endpoint
	 * stalled). For control endpoints: control request not supported. */
	LIBUSB_TRANSFER_STALL,

	/** Device was disconnected */
	LIBUSB_TRANSFER_NO_DEVICE,

	/** Device sent more data than requested */
	LIBUSB_TRANSFER_OVERFLOW,
}

/** \ingroup asyncio
 * Isochronous packet descriptor. */
pub struct libusb_iso_packet_descriptor {
	/** Length of data to request in this packet */
	length: c_uint,

	/** Amount of data that was actually transferred */
	actual_length: c_uint,

	/** Status code for this packet */
	status: libusb_transfer_status,
}

type libusb_callback_fn = extern "C" fn(*mut libusb_transfer);

/** \ingroup asyncio
 * The generic USB transfer structure. The user populates this structure and
 * then submits it in order to request a transfer. After the transfer has
 * completed, the library populates the transfer with the results and passes
 * it back to the user.
 */
pub struct libusb_transfer {
	/** Handle of the device that this transfer will be submitted to */
	pub dev_handle: *mut libusb_device_handle,

	/** A bitwise OR combination of \ref libusb_transfer_flags. */
	pub flags: uint8_t,

	/** Address of the endpoint where this transfer will be sent. */
	pub endpoint: uint8_t,

	/** Type of the endpoint from \ref libusb_transfer_type

	    Note: name differs from libusb because `type` is a Rust keyword.
	*/
	pub transfer_type: uint8_t,

	/** Timeout for this transfer in millseconds. A value of 0 indicates no
	 * timeout. */
	pub timeout: c_uint,

	/** The status of the transfer. Read-only, and only for use within
	 * transfer callback function.
	 *
	 * If this is an isochronous transfer, this field may read COMPLETED even
	 * if there were errors in the frames. Use the
	 * \ref libusb_iso_packet_descriptor::status "status" field in each packet
	 * to determine if errors occurred. */
	pub status: c_uint,

	/** Length of the data buffer */
	pub length: c_int,

	/** Actual length of data that was transferred. Read-only, and only for
	 * use within transfer callback function. Not valid for isochronous
	 * endpoints. */
	pub actual_length: c_int,

	/** Callback function. This will be invoked when the transfer completes,
	 * fails, or is cancelled. */
	pub callback: libusb_callback_fn,

	/** User context data to pass to the callback function. */
	pub user_data: *mut c_void,

	/** Data buffer */
	pub buffer: *mut uint8_t,

	/** Number of isochronous packets. Only used for I/O with isochronous
	 * endpoints. */
	pub num_iso_packets: c_int,

	// /** Isochronous packet descriptors, for isochronous transfers only. */
	//struct libusb_iso_packet_descriptor iso_packet_desc;
}

impl libusb_transfer {
	#[inline]
	pub fn get_status(&self) -> libusb_transfer_status {
		num::FromPrimitive::from_u32(self.status).unwrap()
	}
}

/** Setup packet for control transfers. */
pub struct libusb_control_setup {
	/** Request type. Bits 0:4 determine recipient, see
	 * \ref libusb_request_recipient. Bits 5:6 determine type, see
	 * \ref libusb_request_type. Bit 7 determines data transfer direction, see
	 * \ref libusb_endpoint_direction.
	 */
	pub bmRequestType: u8,

	/** Request. If the type bits of bmRequestType are equal to
	 * \ref libusb_request_type::LIBUSB_REQUEST_TYPE_STANDARD
	 * "LIBUSB_REQUEST_TYPE_STANDARD" then this field refers to
	 * \ref libusb_standard_request. For other cases, use of this field is
	 * application-specific. */
	pub bRequest: u8,

	/** Value. Varies according to request */
	pub wValue: u16,

	/** Index. Varies according to request, typically used to pass an index
	 * or offset */
	pub wIndex: u16,

	/** Number of bytes to transfer */
	pub wLength: u16
}
