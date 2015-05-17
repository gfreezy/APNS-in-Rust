mod notification;
mod payload;


extern crate openssl;
extern crate byteorder;
extern crate num;
extern crate rustc_serialize;


use openssl::ssl;
use openssl::ssl::error::SslError;
use openssl::ssl::SslStream;
use std::net::TcpStream;
use std::env;
use std::io::Write;
use byteorder::{BigEndian, ReadBytesExt};

use notification::{Notification, BatchNotifications};
use payload::{Payload, PayloadAPS, PayloadAPSAlert};


fn connect() -> Result<SslStream<TcpStream>, SslError> {
	let cwd = env::current_dir().unwrap();
	let cert_file = &cwd.join("Certificates.pem");


	let mut context = try!(ssl::SslContext::new(ssl::SslMethod::Tlsv1));
	if let Err(error) = context.set_certificate_file(cert_file, openssl::x509::X509FileType::PEM) {
		println!("set_certificate_file error {:?}", error);
	}
	if let Err(error) = context.set_private_key_file(cert_file, openssl::x509::X509FileType::PEM) {
		println!("set_private_key_file error {:?}", error);
	}

	let tcp_conn = match TcpStream::connect("gateway.push.apple.com:2195") {
		Ok(conn) => conn,
		Err(error) => {
			println!("tcp_stream connect error {:?}", error);
			return Result::Err(SslError::StreamError(error));
		}
	};

	return SslStream::new(&context, tcp_conn);
}


fn read_response(ssl: &mut SslStream<TcpStream>) -> byteorder::Result<(u8, u8, u32)> {

	let command = try!(ssl.read_u8());
	let status = try!(ssl.read_u8());
	let identifier = try!(ssl.read_u32::<BigEndian>());

	Ok((command, status, identifier))
}


fn find_error(ssl: &mut SslStream<TcpStream>) {
	let resp = read_response(ssl);
	match resp {
		Ok((command, status, identifier)) => println!("command {:?}, status {:?}, identifier {:?}", command, status, identifier),
		Err(error) => println!("read response error {:?}", error)
	};
}

fn main() {
	let alert = PayloadAPSAlert::Plain("Hello world");
	let aps = PayloadAPS{alert: alert, badge: None, sound: Some("default"), content_available: None};
	let payload = Payload{aps: aps, info: None};
	let device_token = "3f048a7fe079a3ae3808d45b70f3013afdd004dad4834b525c6ac257ffa4e8cb";
	let noti = Notification{device_token: device_token, payload: &payload, identifier: 1, expire_time: 0};
	let noti2 = Notification{
		device_token: device_token,
		payload: &Payload{
			aps: PayloadAPS{
				alert: PayloadAPSAlert::Plain("Hello 2"),
				badge: None,
				sound: Some("default"),
				content_available: None
			},
			info: None
		},
		identifier: 1,
		expire_time: 0
	};
	let batch_notis = BatchNotifications(vec![noti, noti2]);
  let notification_bytes = batch_notis.to_bytes();

	let mut ssl = match connect() {
		Ok(ssl) => ssl,
		Err(error) => {
				println!("connect {:?}", error);
				return;
		}
	};

	if let Err(err) = ssl.write_all(&notification_bytes) {
		println!("send1 {:?}", err);
		find_error(&mut ssl);
		return;
	};

	if let Err(err) = ssl.flush() {
		println!("flush1 {:?}", err);
		find_error(&mut ssl);
		return;
	}

	find_error(&mut ssl);
}
