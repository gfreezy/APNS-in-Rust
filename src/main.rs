extern crate openssl;
extern crate byteorder;
extern crate num;
extern crate rustc_serialize;
extern crate toml;

mod notification;
mod payload;
mod config;
mod connection;


use std::env;
use notification::{Notification, BatchNotifications};
use payload::{Payload, PayloadAPS, PayloadAPSAlert};


fn main() {
	let cwd = env::current_dir().ok().expect("get current dir");
	let config_path = cwd.join("config.toml");
	let config = config::Config::new(config_path);

	let mut conn = connection::Connection::new(
		config.get_apns_host().to_string(),
		config.get_cert_path().to_path_buf(),
		config.get_cert_type(),
	);

	let device_token = "3f048a7fe079a3ae3808d45b70f3013afdd004dad4834b525c6ac257ffa4e8cb";
	let noti = Notification {
		device_token: device_token.to_string(),
		payload: Payload {
			aps: PayloadAPS {
				alert: PayloadAPSAlert::Plain("Hello world bb2".to_string()),
				badge: None,
				sound: Some("default".to_string()),
				content_available: None
			},
			info: None
		},
		identifier: 1130,
		expire_time: 0
	};

	let noti2 = Notification {
		device_token: device_token.to_string(),
		payload: Payload{
			aps: PayloadAPS{
				alert: PayloadAPSAlert::Plain("Hello aa3".to_string()),
				badge: None,
				sound: Some("default".to_string()),
				content_available: None
			},
			info: None
		},
		identifier: 1100,
		expire_time: 0
	};

	let notis = vec![noti, noti2];
	let batch_notis = BatchNotifications(&notis);

	match conn.send_batch_notifications(&batch_notis) {
		Ok(len) => {
			println!("succeed {:?}", len);
		},
		Err(err) => println!("{:?}", err)
	};
}
