use payload::Payload;
use super::rustc_serialize::{Encodable, json};
use super::rustc_serialize::hex::FromHex;
use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;


const DEVICE_TOKEN_ITEM_ID: u8 = 1;
const PAYLOAD_ITEM_ID: u8 = 2;
const NOTIFICATION_IDENTIFIER_ITEM_ID:u8 = 3;
const EXPIRATION_DATE_ITEM_ID: u8 = 4;
const PRIORITY_ITEM_ID: u8 = 5;
const DEVICE_TOKEN_LENGTH: u16 = 32;
const NOTIFICATION_IDENTIFIER_LENGTH:u16 = 4;
const EXPIRATION_DATE_LENGTH: u16 = 4;
const PRIORITY_LENGTH: u16 = 1;
const PUSH_COMMAND_VALUE: u8 = 2;


#[derive(Debug, RustcEncodable)]
pub struct Notification<'a> {
    pub device_token: &'a str,
    pub payload: &'a Payload<'a>,
    pub identifier: u32,
    pub expire_time: u32
}


impl<'a> Notification<'a> {

	pub fn to_bytes(&self) -> Vec<u8> {
		let mut message_buffer: Vec<u8> = vec![];
		let payload = self.payload;
		let payload_str: String = match json::encode(payload) {
			Ok(json_str) => json_str.to_string(),
			Err(err) => {
				println!("json encode error {:?}", err);
				return message_buffer;
			}
		};

		let payload_bytes = payload_str.into_bytes();
		let device_token_bytes = self.device_token.from_hex().unwrap();

		// Device token
		message_buffer.write_u8(DEVICE_TOKEN_ITEM_ID);
		message_buffer.write_u16::<BigEndian>(DEVICE_TOKEN_LENGTH);
		message_buffer.write_all(&device_token_bytes);

		// Payload
		message_buffer.write_u8(PAYLOAD_ITEM_ID);
		message_buffer.write_u16::<BigEndian>(payload_bytes.len() as u16);
		message_buffer.write_all(&payload_bytes);

		// Notification identifier
		message_buffer.write_u8(NOTIFICATION_IDENTIFIER_ITEM_ID);
		message_buffer.write_u16::<BigEndian>(NOTIFICATION_IDENTIFIER_LENGTH);
		message_buffer.write_u32::<BigEndian>(self.identifier);

		// Expiration date
		message_buffer.write_u8(EXPIRATION_DATE_ITEM_ID);
		message_buffer.write_u16::<BigEndian>(EXPIRATION_DATE_LENGTH);
		message_buffer.write_u32::<BigEndian>(self.expire_time);

		// Priority
		message_buffer.write_u8(PRIORITY_ITEM_ID);
		message_buffer.write_u16::<BigEndian>(PRIORITY_LENGTH);
		message_buffer.write_u8(10u8);

		return message_buffer;
	}
}


pub struct BatchNotifications<'a>(pub Vec<Notification<'a>>);

impl<'a> BatchNotifications<'a> {
	pub fn to_bytes(&self) -> Vec<u8> {
		let BatchNotifications(ref notification_list) = *self;
		let message_buffers: Vec<Vec<u8>> = notification_list.iter().map(|noti| noti.to_bytes()).collect();
		let mut notification_total_buffer = vec![];
		for buf in message_buffers.iter() {
			notification_total_buffer.write_all(&buf);
		}
		let mut notification_bytes: Vec<u8> = vec![];
		notification_bytes.write_u8(PUSH_COMMAND_VALUE);
		notification_bytes.write_u32::<BigEndian>(notification_total_buffer.len() as u32);
		notification_bytes.write_all(&notification_total_buffer);

		return notification_bytes;
	}
}
