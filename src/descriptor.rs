/// Descriptor type, as used in the second byte of a descriptor or high byte of
/// `wValue` in a `GET_DESCRIPTOR` request.
///
/// Universal Serial Bus Specification Revision 2.0, Table 9-5\
/// Universal Serial Bus 3.2 Specification, Table 9-6
pub mod descriptor_type {
    // USB 2.0 spec
    pub const DEVICE: u8 = 1;
    pub const CONFIGURATION: u8 = 2;
    pub const STRING: u8 = 3;
    pub const INTERFACE: u8 = 4;
    pub const ENDPOINT: u8 = 5;
    pub const DEVICE_QUALIFIER: u8 = 6;
    pub const OTHER_SPEED_CONFIGURATION: u8 = 7;
    pub const INTERFACE_POWER: u8 = 8;

    // USB 3.0 spec
    pub const OTG: u8 = 9;
    pub const DEBUG: u8 = 10;
    pub const INTERFACE_ASSOCIATION: u8 = 11;
    pub const BOS: u8 = 15;
    pub const DEVICE_CAPABILITY: u8 = 16;
    pub const SUPERSPEED_USB_ENDPOINT_COMPANION: u8 = 48;
    pub const SUPERSPEEDPLUS_ISOCHRONOUS_ENDPOINT_COMPANION: u8 = 49;
}

/// Base class codes, as used in the `bDeviceClass` of a device descriptor or
/// `bInterfaceClass` of an interface descriptor.
///
/// * [Defined Class Codes](https://usb.org/defined-class-codes)
pub mod class_code {
    /// Used on the device level when class is defined per-interface
    pub const DEVICE: u8 = 0x00;
    pub const AUDIO: u8 = 0x01;
    pub const COMMUNICATION: u8 = 0x02;
    pub const HID: u8 = 0x03;
    pub const PHYSICAL: u8 = 0x05;
    pub const STILL_IMAGING: u8 = 0x06;
    pub const PRINTER: u8 = 0x07;
    pub const MASS_STORAGE: u8 = 0x08;
    pub const HUB: u8 = 0x09;
    pub const SMART_CARD: u8 = 0x0B;
    pub const CONTENT_SECURITY: u8 = 0x0D;
    pub const VIDEO: u8 = 0x0E;
    pub const PERSONAL_HEALTHCARE: u8 = 0x0F;
    pub const DIAGNOSTIC: u8 = 0xDC;
    pub const WIRELESS: u8 = 0xE0;
    pub const MISCELLANEOUS: u8 = 0xEF;
    pub const APPLICATION: u8 = 0xFE;
    pub const VENDOR_SPECIFIC: u8 = 0xFF;
}

/// Language ID, as used in `wIndex` in a `GET_DESCRIPTOR` request
pub mod language_id {
    pub const ENGLISH_US: u16 = 0x0409;
}
