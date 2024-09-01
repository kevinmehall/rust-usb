/// `bEndpointAddress` field of the endpoint descriptor.
pub mod endpoint_address {
    /// Bit 3..0 of endpoint `bEndpointAddress`: Address
    pub const ADDR_MASK: u8 = 0x0F;

    /// Bit 7 of endpoint `bEndpointAddress`: Direction
    pub const DIR_MASK: u8 = 0x80;

    /// Out: Host to device
    pub const OUT: u8 = 0x00;

    /// In: Device to host
    pub const IN: u8 = 0x80;
}

/// `bmAttributes` field of the endpoint descriptor.
pub mod endpoint_attributes {
    /// Mask for bits 1..0.
    pub const TRANSFER_TYPE_MASK: u8 = 0b11;

    /// Bits 1..0: Transfer Type.
    pub mod transfer_type {
        pub const CONTROL: u8 = 0b00;
        pub const ISOCHRONOUS: u8 = 0b01;
        pub const BULK: u8 = 0b10;
        pub const INTERRUPT: u8 = 0b11;
    }

    /// Mask for bits 3..2.
    pub const SYNCHRONIZATION_MASK: u8 = 0b11 << 2;

    /// Bits 3..2: Synchronization Type of an isochronous endpoint.
    pub mod synchronization {
        pub const NO_SYNCHRONIZATION: u8 = 0b00 << 2;
        pub const ASYNCHRONOUS: u8 = 0b01 << 2;
        pub const ADAPTIVE: u8 = 0b10 << 2;
        pub const SYNCHRONOUS: u8 = 0b11 << 2;
    }

    /// Mask for bits 5..4.
    pub const USAGE_MASK: u8 = 0b11 << 4;

    /// Bits 5..4: Usage Type of an isochronous endpoint.
    pub mod usage {
        pub const DATA_ENDPOINT: u8 = 0b00 << 4;
        pub const FEEDBACK_ENDPOINT: u8 = 0b01 << 4;
        pub const IMPLICIT_FEEDBACK: u8 = 0b10 << 4;
    }
}
