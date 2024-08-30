/// `bmRequestType` field of the SETUP packet.
pub mod request_type {
    /// Mask for bit 7 of `bmRequestType`.
    pub const DIRECTION_MASK: u8 = 0x80;

    /// Bit 7 of `bmRequestType`: Direction field.
    pub mod direction {
        pub const IN: u8 = 0x80;
        pub const OUT: u8 = 0x00;
    }

    /// Mask for bits 6..5 of `bmRequestType`.
    pub const REQUEST_TYPE_MASK: u8 = 0b11 << 5;

    /// Bits 6..5 of `bmRequestType`: Request type.
    pub mod request_type {
        /// Request defined by USB standard
        pub const STANDARD: u8 = 0 << 5;

        /// Request defined by class
        pub const CLASS: u8 = 1 << 5;

        /// Request defined by vendor
        pub const VENDOR: u8 = 2 << 5;
    }

    /// Mask for bits 4..0 of `bmRequestType`
    pub const RECIPIENT_MASK: u8 = 0b11111;

    /// Bits 6..5 of `bmRequestType`: Recipient type.
    pub mod recipient {
        /// Request is targeting the device as a whole.
        pub const DEVICE: u8 = 0;

        /// Request is targeting the interface specified in `wIndex`.
        pub const INTERFACE: u8 = 1;

        /// Request is targeting the  endpoint specified in `wIndex`.
        pub const ENDPOINT: u8 = 2;

        /// Other
        pub const OTHER: u8 = 3;
    }
}

/// Standard requests, as used in the `bRequest` field of a setup packet.
///
/// Universal Serial Bus Specification Revision 2.0, Table 9-4\
/// Universal Serial Bus 3.2 Specification, Table 9-5
pub mod standard_request {
    // USB 2.0
    pub const GET_STATUS: u8 = 0;
    pub const CLEAR_FEATURE: u8 = 1;
    pub const SET_FEATURE: u8 = 3;
    pub const SET_ADDRESS: u8 = 5;
    pub const GET_DESCRIPTOR: u8 = 6;
    pub const SET_DESCRIPTOR: u8 = 7;
    pub const GET_CONFIGURATION: u8 = 8;
    pub const SET_CONFIGURATION: u8 = 9;
    pub const GET_INTERFACE: u8 = 10;
    pub const SET_INTERFACE: u8 = 11;
    pub const SYNCH_FRAME: u8 = 12;

    // USB 3.0
    pub const SET_ENCRYPTION: u8 = 13;
    pub const GET_ENCRYPTION: u8 = 14;
    pub const SET_HANDSHAKE: u8 = 15;
    pub const GET_HANDSHAKE: u8 = 16;
    pub const SET_CONNECTION: u8 = 17;
    pub const SET_SECURITY_DATA: u8 = 18;
    pub const GET_SECURITY_DATA: u8 = 19;
    pub const SET_WUSB_DATA: u8 = 20;
    pub const LOOPBACK_DATA_WRITE: u8 = 21;
    pub const LOOPBACK_DATA_READ: u8 = 22;
    pub const SET_INTERFACE_DS: u8 = 23;
    pub const GET_FW_STATUS: u8 = 26;
    pub const SET_FW_STATUS: u8 = 27;
    pub const SET_SEL: u8 = 48;
    pub const SET_ISOCH_DELAY: u8 = 49;
}

/// Feature selector, as used in `wValue` of a `SET_FEATURE` or `CLEAR_FEATURE`
/// request.
///
/// Universal Serial Bus Specification Revision 2.0, Table 9-6
pub mod feature_selector {
    pub const ENDPOINT_HALT: u16 = 0;
    pub const DEVICE_REMOTE_WAKEUP: u16 = 1;
    pub const TEST_MODE: u16 = 2;
}

/// Test mode, as used in the upper byte of `wIndex` of a `SET_FEATURE` request.
///
/// Universal Serial Bus Specification Revision 2.0, Table 9-7
pub mod test_mode {
    pub const TEST_J: u8 = 0x01;
    pub const TEST_K: u8 = 0x02;
    pub const TEST_SE0_NAK: u8 = 0x03;
    pub const TEST_PACKET: u8 = 0x04;
    pub const TEST_FORCE_ENABLE: u8 = 0x05;
}
