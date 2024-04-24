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

use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_s3::config::Credentials;
use serde::{de::DeserializeOwned, Deserialize};
use toml::Value;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    rust_log: String,
    pub database_url: String,
    object_storage: ObjectStorageConfig,
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
                .unwrap_or(Value::Table(toml::Table::new()));

            Self::load_config(&[], &config_toml)
        })
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }

    pub fn env_filter(&self) -> EnvFilter {
        EnvFilter::new(&self.rust_log)
    }

    pub async fn s3_config(&self) -> SdkConfig {
        aws_config::defaults(BehaviorVersion::v2023_11_09())
            .region("eu-central-1")
            .endpoint_url(&self.object_storage.endpoint_url)
            .credentials_provider(Credentials::new(
                &self.object_storage.access_key_id,
                &self.object_storage.secret_access_key,
                self.object_storage.session_token.clone(),
                None,
                "tortoaster-credential-provider",
            ))
            .load()
            .await
    }

    pub fn buckets(&self) -> &BucketConfig {
        &self.object_storage.bucket
    }
}

impl LoadConfig for AppConfig {
    fn load_config(prefixes: &[&'static str], config_toml: &Value) -> Self {
        AppConfig {
            host: retrieve(
                &[],
                "host",
                "ip address",
                config_toml,
                Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
            ),
            port: retrieve(&[], "port", "port number", config_toml, Some(8000)),
            rust_log: retrieve(
                &[],
                "rust_log",
                "log specification",
                config_toml,
                Some(String::new()),
            ),
            database_url: retrieve(&[], "database_url", "url", config_toml, None),
            object_storage: ObjectStorageConfig::load_config(
                &prefixes
                    .iter()
                    .copied()
                    .chain(Some("object_storage"))
                    .collect::<Vec<_>>(),
                &config_toml["object_storage"],
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ObjectStorageConfig {
    pub endpoint_url: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub bucket: BucketConfig,
}

impl LoadConfig for ObjectStorageConfig {
    fn load_config(prefixes: &[&'static str], config_toml: &Value) -> Self {
        ObjectStorageConfig {
            endpoint_url: retrieve(prefixes, "endpoint_url", "url", config_toml, None),
            access_key_id: retrieve(prefixes, "access_key_id", "id", config_toml, None),
            secret_access_key: retrieve(prefixes, "secret_access_key", "key", config_toml, None),
            session_token: try_retrieve(prefixes, "session_token", "token", config_toml),
            bucket: BucketConfig::load_config(
                &prefixes
                    .iter()
                    .copied()
                    .chain(Some("bucket"))
                    .collect::<Vec<_>>(),
                &config_toml["bucket"],
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BucketConfig {
    pub thumbnails: String,
    pub content: String,
}

impl LoadConfig for BucketConfig {
    fn load_config(prefixes: &[&'static str], config_toml: &Value) -> Self {
        BucketConfig {
            thumbnails: retrieve(prefixes, "thumbnails", "bucket id", config_toml, None),
            content: retrieve(prefixes, "content", "bucket id", config_toml, None),
        }
    }
}

// TODO: Derive or replace with library
trait LoadConfig {
    fn load_config(prefixes: &[&'static str], config_toml: &Value) -> Self;
}

fn try_retrieve<T>(
    prefixes: &[&'static str],
    field_name: &'static str,
    type_name: &'static str,
    config_toml: &Value,
) -> Option<T>
where
    T: FromStr + DeserializeOwned,
    T::Err: Debug,
{
    let env_var_name = &env_var_name(prefixes, field_name);

    env::var(env_var_name)
        .ok()
        .map(|var| {
            var.parse().unwrap_or_else(|_| {
                panic!("environment variable {env_var_name} must be a valid {type_name}")
            })
        })
        .or(config_toml.get(field_name).cloned().map(|value| {
            value.try_into().unwrap_or_else(|_| {
                panic!("Config.toml entry {field_name} must be a valid {type_name}")
            })
        }))
}

fn retrieve<T>(
    prefixes: &[&'static str],
    field_name: &'static str,
    type_name: &'static str,
    config_toml: &Value,
    default: Option<T>,
) -> T
where
    T: FromStr + DeserializeOwned,
    T::Err: Debug,
{
    try_retrieve(prefixes, field_name, type_name, config_toml)
        .or(default)
        .unwrap_or_else(|| {
            let env_var_name = env_var_name(prefixes, field_name);

            panic!(
                "config must contain {field_name}, add it to `Config.toml` or set the \
                 {env_var_name} environment variable"
            )
        })
}

fn env_var_name(prefixes: &[&'static str], field_name: &'static str) -> String {
    let prefix: String = prefixes
        .iter()
        .copied()
        .map(str::to_uppercase)
        .map(|prefix| prefix + "_")
        .collect();

    format!("{}{}", prefix, field_name.to_uppercase())
}
