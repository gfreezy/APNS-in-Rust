use super::rustc_serialize::{Encodable, Encoder};
use std::collections::HashMap;


#[derive(Debug)]
pub struct Payload<'a> {
	pub aps: PayloadAPS<'a>,
	pub info: Option<HashMap<&'a str, &'a str>>
}

#[derive(Debug)]
pub struct PayloadAPS<'a> {
	pub alert: PayloadAPSAlert<'a>,
	pub badge: Option<i32>,
	pub sound: Option<&'a str>,
	pub content_available: Option<i32>
}

#[derive(Debug)]
pub enum PayloadAPSAlert<'a> {
	Plain(&'a str),
	Localized(&'a str, Vec<&'a str>)
}

impl<'a> Encodable for Payload<'a> {
	fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
		match *self {
			Payload{ref aps, ref info} => {
				if let Some(ref map) = *info {
					encoder.emit_struct("Payload", 1 + map.len(),
						|encoder| {
							try!(encoder.emit_struct_field( "aps", 0usize, |encoder| aps.encode(encoder)));
							let mut index = 1usize;
							for (key, val) in map.iter() {
								try!(encoder.emit_struct_field(key, index, |encoder| val.encode(encoder)));
								index = index + 1;
							}
							Ok(())
					})
				}

				else {
					encoder.emit_struct("Payload", 1, |encoder| {
						try!(encoder.emit_struct_field( "aps", 0usize, |encoder| aps.encode(encoder)));
						Ok(())
					})
				}
			}
		}
	}
}

impl<'a> Encodable for PayloadAPS<'a> {
	fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
			match *self {
			PayloadAPS{ref alert, ref badge, ref sound, ref content_available} => {
				let mut count = 1;
				if badge.is_some() { count = count + 1; }
				if sound.is_some() { count = count + 1; }
				if content_available.is_some() { count = count + 1; }

				let mut index = 0usize;
				encoder.emit_struct("PayloadAPS", count, |encoder| {
					try!(encoder.emit_struct_field( "alert", index, |encoder| alert.encode(encoder)));
					index = index + 1;
					if badge.is_some() {
						try!(encoder.emit_struct_field( "badge", index, |encoder| badge.unwrap().encode(encoder)));
						index = index + 1;
					}
					if sound.is_some() {
						try!(encoder.emit_struct_field( "sound", index, |encoder| sound.unwrap().encode(encoder)));
						index = index + 1;
					}
					if content_available.is_some() {
						try!(encoder.emit_struct_field( "content-available", index, |encoder| content_available.unwrap().encode(encoder)));
						index = index + 1;
					}
					Ok(())
				})
			}
		}
	}
}

impl<'a> Encodable for PayloadAPSAlert<'a> {
	fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
			match *self {
			PayloadAPSAlert::Plain(ref str) => {
				encoder.emit_str(str)
			},
			PayloadAPSAlert::Localized(ref key, ref args) => {
				encoder.emit_struct("PayloadAPSAlert", 2, |encoder| {
				try!(encoder.emit_struct_field( "loc-key", 0usize, |encoder| key.encode(encoder)));
					try!(encoder.emit_struct_field( "loc-args", 1usize, |encoder| args.encode(encoder)));
					Ok(())
				})
			}
		}
	}
}
