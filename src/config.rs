use std::{
    fmt::Debug,
    net::{IpAddr, SocketAddr},
    sync::OnceLock,
};

use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_s3::config::Credentials;
use config::Config;
use serde::Deserialize;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    rust_log: String,
    pub database_url: String,
    object_storage: ObjectStorageConfig,
    pub oidc: OidcConfig,
}

impl AppConfig {
    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<AppConfig> = OnceLock::new();

        CONFIG.get_or_init(|| {
            let settings = Config::builder()
                .add_source(config::File::with_name("Config.toml"))
                .add_source(config::Environment::default())
                .build()
                .expect("invalid config settings");

            settings
                .try_deserialize()
                .expect("failed to deserialize config")
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

#[derive(Debug, Deserialize)]
pub struct ObjectStorageConfig {
    pub endpoint_url: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub bucket: BucketConfig,
}

#[derive(Debug, Deserialize)]
pub struct BucketConfig {
    pub thumbnails: String,
    pub content: String,
    pub system: String,
}

#[derive(Debug, Deserialize)]
pub struct OidcConfig {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub issuer_url: String,
    pub redirect_url: String,
}
