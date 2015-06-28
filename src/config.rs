use std;
use std::io::Read;
use toml;
use openssl;


#[allow(dead_code)]
pub struct Config {
  path: std::path::PathBuf,
  config: toml::Table,
}

impl Config {
  pub fn new(path: std::path::PathBuf) -> Config {
    let config = Config::read_config(&path);

    Config {
      path: path,
      config: config,
    }
  }

  pub fn get_cert_path(&self) -> &std::path::Path {
    if let &toml::Value::String(ref path) = self.config.get("cert_path").expect("cert_path not exist") {
      &std::path::Path::new(path)
    } else {
      panic!("no cert path");
    }
  }

  pub fn get_cert_type(&self) -> openssl::x509::X509FileType {
    if let Some(cert_type) = self.config.get("cert_type") {
      if let toml::Value::String(ref s) = *cert_type {
        return match s.as_ref() {
          "pem" => openssl::x509::X509FileType::PEM,
          _ => openssl::x509::X509FileType::PEM
        };
      }
    }

    openssl::x509::X509FileType::PEM
  }

  pub fn get_apns_host(&self) -> &str {
    let apns_host = match self.config.get("apns_host") {
      None => {
        return "gateway.push.apple.com:2195";
      },
      Some(v) => v
    };

    if let toml::Value::String(ref host) = *apns_host {
      return host.as_ref();
    }
    "gateway.push.apple.com:2195"
  }

  #[allow(dead_code)]
  pub fn get_config_path(&self) -> &std::path::Path {
    self.path.as_path()
  }

  fn read_config(path: &std::path::Path) -> toml::Table {
    let mut file = std::fs::File::open(path).ok().expect("open config error");
    let mut config_content = String::with_capacity(100);
    let read_size = file.read_to_string(&mut config_content).ok().expect("read config error");
    if read_size == 0 {
      panic!("config empty");
    }
    let mut parser = toml::Parser::new(&config_content);
    match parser.parse() {
      Some(table) => table,
      None => panic!("parse config error")
    }
  }
}
