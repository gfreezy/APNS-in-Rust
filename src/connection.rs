use std;
use openssl;
use std::net::TcpStream;
use std::io::Write;
use notification;
use openssl::ssl;
use openssl::ssl::error::SslError;
use openssl::ssl::SslStream;
use byteorder::{BigEndian, ReadBytesExt};

pub struct Connection {
	#[warn(dead_code)]
  cert_path: std::path::PathBuf,
  cert_type: openssl::x509::X509FileType,
  host: String,
  ssl_stream: SslStream<TcpStream>,
}

#[derive(Debug)]
pub enum SendingNotificationError {
	SslError(SslError),
	NotificationError(u8, u8, u32),
}


impl Connection {
	pub fn new(host: String, cert_path: std::path::PathBuf, cert_type: openssl::x509::X509FileType) -> Connection {
		let conn = Connection::connect(&host, &cert_path, cert_type);
		Connection {
			cert_path: cert_path,
			cert_type: cert_type,
			host: host,
			ssl_stream: conn.ok().expect("connect error"),
		}
	}

	pub fn send_batch_notifications(&mut self, batch_notis: &notification::BatchNotifications) -> Result<usize, SendingNotificationError> {
	  let notification_bytes = batch_notis.to_bytes();

		if let Err(err) = self.ssl_stream.write_all(&notification_bytes) {
			return match self.read_response() {
				Ok((a, b, c)) => Err(SendingNotificationError::NotificationError(a, b, c)),
				Err(..) => Err(SendingNotificationError::SslError(SslError::StreamError(err)))
			};
		}

		if let Err(err) = self.ssl_stream.flush() {
			return match self.read_response() {
				Ok((a, b, c)) => Err(SendingNotificationError::NotificationError(a, b, c)),
				Err(..) => Err(SendingNotificationError::SslError(SslError::StreamError(err)))
			};
		}

		Ok(batch_notis.len())
	}

	fn connect(host: &str, cert_path: &std::path::Path, cert_type: openssl::x509::X509FileType) -> Result<SslStream<TcpStream>, SslError> {
		let mut context = ssl::SslContext::new(ssl::SslMethod::Tlsv1).ok().expect("create ssl conext");
		context.set_certificate_file(cert_path, cert_type).ok().expect("set cert file");
		context.set_private_key_file(cert_path, cert_type).ok().expect("set private key");
		let tcp_conn = match TcpStream::connect(host) {
			Ok(conn) => conn,
			Err(error) => {
				return Err(SslError::StreamError(error));
			}
		};

		return SslStream::new(&context, tcp_conn);
	}

	fn read_response(&mut self) -> Result<(u8, u8, u32), SslError> {
		let command = match self.ssl_stream.read_u8() {
			Ok(command) => command,
			Err(err) => {
				return Err(SslError::StreamError(std::io::Error::from(err)));
			}
		};
		let status = match self.ssl_stream.read_u8() {
			Ok(status) => status,
			Err(err) => {
				return Err(SslError::StreamError(std::io::Error::from(err)));
			}
		};
		let identifier = match self.ssl_stream.read_u32::<BigEndian>() {
			Ok(identifier) => identifier,
			Err(err) => {
				return Err(SslError::StreamError(std::io::Error::from(err)));
			}
		};

		Ok((command, status, identifier))
	}
}
