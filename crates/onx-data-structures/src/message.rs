//! Core message structures per docs/specification/data-structures.md §4.3.

use crate::address::FullAddress;
use crate::error::DataStructureError;
use onx_primitives::{Uint128, Uint256, Uint64};

/// Message type discriminant byte tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageType {
    /// Internal message between accounts (0x01).
    Internal = 0x01,
    /// External inbound message from outside network (0x02).
    ExternalInbound = 0x02,
    /// External outbound message to outside network (0x03).
    ExternalOutbound = 0x03,
}

impl MessageType {
    /// Converts a byte tag to MessageType or returns an error.
    pub fn from_tag(tag: u8) -> Result<Self, DataStructureError> {
        match tag {
            0x01 => Ok(Self::Internal),
            0x02 => Ok(Self::ExternalInbound),
            0x03 => Ok(Self::ExternalOutbound),
            _ => Err(DataStructureError::InvalidMessageType { tag }),
        }
    }

    /// Returns byte tag value.
    pub const fn tag(&self) -> u8 {
        *self as u8
    }
}

/// Core Message structure (`Message`).
///
/// Binary layout (129 bytes fixed header/summary):
/// 1. `msg_type`: uint8 (1 byte: 0x01 = Internal, 0x02 = External Inbound, 0x03 = External Outbound)
/// 2. `src_address`: FullAddress (36 bytes)
/// 3. `dest_address`: FullAddress (36 bytes)
/// 4. `amount_nanos`: uint128 (16 bytes, big-endian)
/// 5. `created_lt`: uint64 (8 bytes, logical time)
/// 6. `body_cell_hash`: uint256 (32 bytes, payload digest)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Message {
    pub msg_type: MessageType,
    pub src_address: FullAddress,
    pub dest_address: FullAddress,
    pub amount_nanos: Uint128,
    pub created_lt: Uint64,
    pub body_cell_hash: Uint256,
}

impl Message {
    /// Exact byte size of fixed message layout (129 bytes).
    pub const BYTE_LENGTH: usize = 1 + 36 + 36 + 16 + 8 + 32;

    /// Serializes Message structure to 129 canonical binary bytes.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_LENGTH] {
        let mut buf = [0u8; Self::BYTE_LENGTH];
        buf[0] = self.msg_type.tag();
        buf[1..37].copy_from_slice(&self.src_address.to_bytes());
        buf[37..73].copy_from_slice(&self.dest_address.to_bytes());
        buf[73..89].copy_from_slice(&self.amount_nanos.encode());
        buf[89..97].copy_from_slice(&self.created_lt.encode());
        buf[97..129].copy_from_slice(&self.body_cell_hash.encode());
        buf
    }

    /// Deserializes Message structure from exactly 129 canonical binary bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DataStructureError> {
        if bytes.len() < Self::BYTE_LENGTH {
            return Err(DataStructureError::TruncatedInput {
                expected: Self::BYTE_LENGTH,
                got: bytes.len(),
            });
        }
        if bytes.len() > Self::BYTE_LENGTH {
            return Err(DataStructureError::TrailingBytes {
                remaining: bytes.len() - Self::BYTE_LENGTH,
            });
        }

        let msg_type = MessageType::from_tag(bytes[0])?;
        let src_address = FullAddress::from_bytes(&bytes[1..37])?;
        let dest_address = FullAddress::from_bytes(&bytes[37..73])?;
        let amount_nanos = Uint128::decode_exact(&bytes[73..89])?;
        let created_lt = Uint64::decode_exact(&bytes[89..97])?;
        let body_cell_hash = Uint256::decode_exact(&bytes[97..129])?;

        Ok(Self {
            msg_type,
            src_address,
            dest_address,
            amount_nanos,
            created_lt,
            body_cell_hash,
        })
    }
}
