#![cfg(feature = "protocol")]
use blake2::{digest::consts::U32, Blake2b, Digest};
use thiserror::Error;

const MAX_LENGTH_PACKET: usize = 127;
const MAX_DEVICE_NAME_LENGTH: usize = 64;

const STANDARD_BLAKE_KEY: &str = "GroepCNetwerk";
const STANDARD_BLAKE_KEY_SIZE: usize = size_of_val(STANDARD_BLAKE_KEY);
const STANDARD_BLAKE_HASH_SIZE: usize = 32;

type StandardBlakeHash = Blake2b<U32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MessageType {
    Get = 128,
    Set = 0,
}

#[derive(Debug)]
enum HashStatus {
    Valid = 1,
    Invalid = 0,
}

#[derive(Debug)]
struct MessageStartByte {
    byte: u8,
    message_type: MessageType,
    length: u8,
}

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Length for message was not valid, has to be < 127")]
    LengthError,
}

impl MessageStartByte {
    pub fn new(message_type: MessageType, length: u8) -> Result<Self, ProtocolError> {
        if length > 127 {
            return Err(ProtocolError::LengthError);
        }
        let mut byte: u8 = 0;
        byte |= length;
        byte |= message_type as u8;
        Ok(MessageStartByte {
            byte,
            message_type,
            length,
        })
    }
}

#[derive(Debug)]
struct MessagePacket {
    message_type: MessageType,
    hash_status: HashStatus,
    length: u8,
    data_bytes: [u8; MAX_LENGTH_PACKET],
    hash: [u8; STANDARD_BLAKE_HASH_SIZE],
}

impl MessagePacket {
    pub fn new(data: &[u8], message_type: MessageType) -> Result<MessagePacket, ProtocolError> {
        let length: u8 = data.len() as u8;
        let type_m: MessageType = message_type;

        let mut hasher = StandardBlakeHash::new();
        let start_byter = MessageStartByte::new(type_m, length)?;
        hasher.update([start_byter.byte]);
        hasher.update(data);

        let hash = hasher
            .finalize()
            .as_slice()
            .try_into()
            .expect("The hasher got the wrong length, how did you achieve that?");

        let mut data_byte: [u8; MAX_LENGTH_PACKET] = [0; 127];
        data_byte[..data.len()].copy_from_slice(data);

        Ok(MessagePacket {
            message_type,
            hash_status: HashStatus::Valid,
            length,
            data_bytes: data_byte,
            hash,
        })
    }
}
