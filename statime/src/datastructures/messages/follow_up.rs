use crate::datastructures::{common::Timestamp, WireFormat, WireFormatError};
use getset::CopyGetters;

use super::Header;

#[derive(Debug, Clone, Copy, PartialEq, Eq, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct FollowUpMessage {
    pub(super) header: Header,
    pub(super) precise_origin_timestamp: Timestamp,
}

impl FollowUpMessage {
    pub fn content_size(&self) -> usize {
        10
    }

    pub fn serialize_content(
        &self,
        buffer: &mut [u8],
    ) -> Result<(), crate::datastructures::WireFormatError> {
        if buffer.len() < 11 {
            return Err(WireFormatError::BufferTooShort);
        }

        self.precise_origin_timestamp
            .serialize(&mut buffer[0..10])?;

        Ok(())
    }

    pub fn deserialize_content(
        header: Header,
        buffer: &[u8],
    ) -> Result<Self, crate::datastructures::WireFormatError> {
        if buffer.len() < 11 {
            return Err(WireFormatError::BufferTooShort);
        }
        Ok(Self {
            header,
            precise_origin_timestamp: Timestamp::deserialize(&buffer[0..10])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_wireformat() {
        let representations = [
            (
                [0x00, 0x00, 0x45, 0xb1, 0x11, 0x5a, 0x0a, 0x64, 0xfa, 0xb0],
                FollowUpMessage {
                    header: Header::default(),
                    precise_origin_timestamp: Timestamp {
                        seconds: 1169232218,
                        nanos: 174389936,
                    },
                },
            ),
            (
                [0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01u8],
                FollowUpMessage {
                    header: Header::default(),
                    precise_origin_timestamp: Timestamp {
                        seconds: 0x0000_0000_0002,
                        nanos: 0x0000_0001,
                    },
                },
            ),
        ];

        for (byte_representation, object_representation) in representations {
            // Test the serialization output
            let mut serialization_buffer = [0; 10];
            object_representation
                .serialize_content(&mut serialization_buffer)
                .unwrap();
            assert_eq!(serialization_buffer, byte_representation);

            // Test the deserialization output
            let deserialized_data =
                FollowUpMessage::deserialize_content(Header::default(), &byte_representation)
                    .unwrap();
            assert_eq!(deserialized_data, object_representation);
        }
    }
}
