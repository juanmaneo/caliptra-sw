/*++

Licensed under the Apache-2.0 license.

File Name:

    doe_regs.rs

Abstract:

    File contains register definitions for Deobfuscation engine

--*/

use crate::reg::static_ref::StaticRef;
use tock_registers::registers::ReadOnly;
use tock_registers::registers::ReadWrite;
use tock_registers::{register_bitfields, register_structs};

register_structs! {
    pub DoeRegisters {
        /// Initialization Vector
        (0x00 => pub(crate) iv: [ReadWrite<u32>; 4]),

        /// Control
        (0x10 => pub(crate) control: ReadWrite<u32, CONTROL::Register>),

        /// Status
        (0x14 => pub(crate) status: ReadOnly<u32, STATUS::Register>),

        (0x18 => @END),
    }
}

register_bitfields! [
    u32,

    /// Control Register Fields
    pub(crate) CONTROL [
        CMD OFFSET(0) NUMBITS(2) [
            IDLE = 0b00,
            DECRYPT_UDS = 0b01,
            DECRYPT_FIELD_ENTROPY = 0b10,
            CLEAR_SECRETS = 0b11,
        ],
        DEST OFFSET(2) NUMBITS(3) [],
    ],

    /// Status Register Fields
    pub(crate) STATUS [
        READY OFFSET(0) NUMBITS(1) [],
        VALID OFFSET(1) NUMBITS(1) [],
    ],
];

/// Deobfuscation engine registers
pub(crate) const DOE_REGS: StaticRef<DoeRegisters> =
    unsafe { StaticRef::new(0x1000_0000 as *const DoeRegisters) };