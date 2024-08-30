/// Device Capability Type codes as used in the `bDevCapabilityType` field of the BOS
/// Device Capability Descriptor.
///
/// Universal Serial Bus 3.2 Specification, Table 9-14
pub mod capability_type {
    pub const WIRELESS_USB: u8 = 0x01;
    pub const USB_2_0_EXTENSION: u8 = 0x02;
    pub const SUPERSPEED_USB: u8 = 0x03;
    pub const CONTAINER_ID: u8 = 0x04;
    pub const PLATFORM: u8 = 0x05;
    pub const POWER_DELIVERY_CAPABILITY: u8 = 0x06;
    pub const BATTERY_INFO_CAPABILITY: u8 = 0x07;
    pub const PD_CONSUMER_PORT_CAPABILITY: u8 = 0x08;
    pub const PD_PROVIDER_PORT_CAPABILITY: u8 = 0x09;
    pub const SUPERSPEED_PLUS: u8 = 0x0A;
    pub const PRECISION_TIME_MEASUREMENT: u8 = 0x0B;
    pub const WIRELESS_USB_EXT: u8 = 0x0C;
    pub const BILLBOARD: u8 = 0x0D;
    pub const AUTHENTICATION: u8 = 0x0E;
    pub const BILLBOARD_EX: u8 = 0x0F;
    pub const CONFIGURATION_SUMMARY: u8 = 0x10;
    pub const FW_STATUS: u8 = 0x11;
}
