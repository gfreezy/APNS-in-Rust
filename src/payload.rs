use rustc_serialize::{Encodable, Encoder};
use std::collections::HashMap;


#[derive(Debug)]
pub struct Payload {
	pub aps: PayloadAPS,
	pub info: Option<HashMap<String, String>>
}

#[derive(Debug)]
pub struct PayloadAPS {
	pub alert: PayloadAPSAlert,
	pub badge: Option<i32>,
	pub sound: Option<String>,
	pub content_available: Option<i32>
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum PayloadAPSAlert {
	Plain(String),
	Localized(PayloadAPSLocalizedAlert)
}

#[derive(Debug)]
pub struct PayloadAPSLocalizedAlert {
	pub title: String,
	pub body: String,
	pub title_loc_key: Option<String>,
	pub title_loc_args: Option<Vec<String>>,
	pub action_loc_key: Option<String>,
	pub loc_key: String,
	pub loc_args: Vec<String>,
	pub launch_image: String
}

impl Encodable for Payload {
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

impl Encodable for PayloadAPS {
	fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
			match *self {
			PayloadAPS{ref alert, ref badge, ref sound, ref content_available} => {
				let mut count = 1;
				if badge.is_some() { count = count + 1; }
				if sound.is_some() { count = count + 1; }
				if content_available.is_some() { count = count + 1; }

				let mut index = 0;
				encoder.emit_struct("PayloadAPS", count, |encoder| {
					try!(encoder.emit_struct_field( "alert", index, |encoder| alert.encode(encoder)));
					index = index + 1;
					if badge.is_some() {
						try!(encoder.emit_struct_field( "badge", index, |encoder| badge.unwrap().encode(encoder)));
						index = index + 1;
					}
					if sound.is_some() {
						try!(encoder.emit_struct_field( "sound", index, |encoder| sound.as_ref().unwrap().encode(encoder)));
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

impl Encodable for PayloadAPSAlert {
	fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
		match *self {
			PayloadAPSAlert::Plain(ref s) => {
				encoder.emit_str(s)
			},
			PayloadAPSAlert::Localized(ref alert) => {
				alert.encode(encoder)
			}
		}
	}
}


impl Encodable for PayloadAPSLocalizedAlert {
	fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
		let PayloadAPSLocalizedAlert {
	    ref title,
		  ref body,
		  ref loc_key,
		  ref loc_args,
		  ref launch_image,
		  title_loc_key: ref some_title_loc_key,
		  title_loc_args: ref some_title_loc_args,
		  action_loc_key: ref some_action_loc_key,
		} = *self;

		let mut count = 5;
		if some_title_loc_key.is_some() {
			count += 1;
		}
		if some_title_loc_args.is_some() {
			count += 1;
		}
		if some_action_loc_key.is_some() {
			count += 1;
		}

		let mut index = 5;
		encoder.emit_struct("PayloadAPSLocalizedAlert", count, |encoder| {
			try!(encoder.emit_struct_field("title", 0, |encoder| title.encode(encoder)));
			try!(encoder.emit_struct_field("body", 1, |encoder| body.encode(encoder)));
			try!(encoder.emit_struct_field("loc-key", 2, |encoder| loc_key.encode(encoder)));
			try!(encoder.emit_struct_field("loc-args", 3, |encoder| loc_args.encode(encoder)));
			try!(encoder.emit_struct_field("launch-image", 4, |encoder| launch_image.encode(encoder)));
			if let Some(ref title_loc_key) = *some_title_loc_key {
				try!(encoder.emit_struct_field("title-loc-key", index, |encoder| title_loc_key.encode(encoder)));
				index += 1;
			}

			if let Some(ref title_loc_args) = *some_title_loc_args {
				try!(encoder.emit_struct_field("title-loc-args", index, |encoder| title_loc_args.encode(encoder)));
				index += 1;
			}

			if let Some(ref action_loc_key) = *some_action_loc_key {
				try!(encoder.emit_struct_field("action-loc-key", index, |encoder| action_loc_key.encode(encoder)));
				index += 1;
			}
			Ok(())
		})
	}
}
