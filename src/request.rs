/// Standard requests, as used in the `bRequest` field of a setup packet.
pub mod standard_request {
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
}

/// Feature selector, as used in `wValue` of a `SET_FEATURE` or `CLEAR_FEATURE`
/// request.
pub mod feature_selector {
    pub const ENDPOINT_HALT: u16 = 0;
    pub const DEVICE_REMOTE_WAKEUP: u16 = 1;
    pub const TEST_MODE: u16 = 2;
}

/// Test mode, as used in the upper byte of `wIndex` of a `SET_FEATURE` request.
pub mod test_mode {
    pub const TEST_J: u8 = 0x01;
    pub const TEST_K: u8 = 0x02;
    pub const TEST_SE0_NAK: u8 = 0x03;
    pub const TEST_PACKET: u8 = 0x04;
    pub const TEST_FORCE_ENABLE: u8 = 0x05;
}
