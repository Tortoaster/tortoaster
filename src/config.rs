use std::{
    env,
    fmt::Debug,
    fs,
    io::ErrorKind,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    str::FromStr,
    sync::OnceLock,
};

use serde::{de::DeserializeOwned, Deserialize};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    pub database_url: String,
    rust_log: String,
}

impl AppConfig {
    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<AppConfig> = OnceLock::new();

        CONFIG.get_or_init(|| {
            let path = env::current_exe().expect("failed to get executable location");
            let mut acc = PathBuf::new();
            let config_toml = path
                .iter()
                .map(|segment| {
                    let previous = acc.clone();
                    acc = acc.join(segment);
                    previous
                })
                .map(|dir| dir.join("Config.toml"))
                .find_map(|path| match fs::read_to_string(path) {
                    Ok(content) => {
                        Some(toml::from_str(&content).expect("Config.toml must contain valid TOML"))
                    }
                    Err(error) => match error.kind() {
                        ErrorKind::NotFound => None,
                        _ => panic!("error reading Config.toml: {error}"),
                    },
                })
                .unwrap_or(toml::Value::Table(toml::Table::new()));

            AppConfig {
                host: retrieve(
                    "host",
                    "ip address",
                    &config_toml,
                    Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
                ),
                port: retrieve("port", "port number", &config_toml, Some(8000)),
                database_url: retrieve("database_url", "url", &config_toml, None),
                rust_log: retrieve(
                    "rust_log",
                    "log specification",
                    &config_toml,
                    Some(String::new()),
                ),
            }
        })
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }

    pub fn env_filter(&self) -> EnvFilter {
        EnvFilter::new(&self.rust_log)
    }
}

fn retrieve<T>(
    field: &'static str,
    ty: &'static str,
    config_toml: &toml::Value,
    default: Option<T>,
) -> T
where
    T: FromStr + DeserializeOwned,
    T::Err: Debug,
{
    let field_uppercase = field.to_uppercase();
    env::var(&field_uppercase)
        .ok()
        .map(|var| {
            var.parse().unwrap_or_else(|_| {
                panic!("environment variable {field_uppercase} must be a valid {ty}")
            })
        })
        .or(config_toml.get(field).cloned().map(|value| {
            value
                .try_into()
                .unwrap_or_else(|_| panic!("Config.toml entry {field} must be a valid {ty}"))
        }))
        .or(default)
        .unwrap_or_else(|| {
            panic!(
                "config must contain {field}, add it to Config.toml or set the {field_uppercase} \
                 environment variable"
            )
        })
}
