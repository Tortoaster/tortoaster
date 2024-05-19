use std::{
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    ops::Deref,
    sync::OnceLock,
};

use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_s3::config::Credentials;
use config::Config;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use strum::EnumIter;
use tower_sessions_redis_store::fred::prelude::{RedisConfig, Server, ServerConfig};
use tracing_subscriber::EnvFilter;

#[serde_inline_default]
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde_inline_default(IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub host: IpAddr,
    #[serde_inline_default(8000)]
    pub port: u16,
    #[serde_inline_default("info".to_owned())]
    rust_log: String,
    database: DatabaseConfig,
    s3: S3Config,
    pub oidc: OidcConfig,
    #[serde(default)]
    cache: CacheConfig,
}

impl AppConfig {
    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<AppConfig> = OnceLock::new();

        CONFIG.get_or_init(|| {
            Config::builder()
                .add_source(config::File::with_name("Config.toml").required(false))
                .add_source(config::Environment::default().separator("__"))
                .build()
                .expect("invalid config settings")
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

    pub fn database_url(&self) -> &str {
        &self.database.url
    }

    pub async fn s3_config(&self) -> SdkConfig {
        aws_config::defaults(BehaviorVersion::v2023_11_09())
            .region("eu-central-1")
            .endpoint_url(&self.s3.endpoint_url)
            .credentials_provider(Credentials::new(
                &self.s3.access_key_id,
                &self.s3.secret_access_key,
                self.s3.session_token.clone(),
                None,
                "tortoaster-credential-provider",
            ))
            .load()
            .await
    }

    pub fn cache_config(&self) -> RedisConfig {
        RedisConfig {
            server: ServerConfig::Centralized {
                server: Server::new(self.cache.host.to_string(), self.cache.port),
            },
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug, Deserialize)]
struct S3Config {
    endpoint_url: String,
    access_key_id: String,
    secret_access_key: String,
    #[serde(default)]
    session_token: Option<String>,
    #[serde(default)]
    bucket: BucketConfig,
}

#[serde_inline_default]
#[derive(Debug, Deserialize)]
pub struct BucketConfig {
    #[serde_inline_default("tortoaster-thumbnails".to_owned())]
    pub thumbnails: String,
    #[serde_inline_default("tortoaster-content".to_owned())]
    pub content: String,
    #[serde_inline_default("tortoaster-system".to_owned())]
    pub system: String,
}

/// Smart pointer to the bucket names stored in the configuration.
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum AppBucket {
    Thumbnails,
    Content,
    System,
}

impl AppBucket {
    pub fn name(&self) -> &'static str {
        match self {
            AppBucket::Thumbnails => "thumbnails",
            AppBucket::Content => "content",
            AppBucket::System => "system",
        }
    }
}

impl Deref for AppBucket {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let config = AppConfig::get();

        match self {
            AppBucket::Thumbnails => &config.s3.bucket.thumbnails,
            AppBucket::Content => &config.s3.bucket.content,
            AppBucket::System => &config.s3.bucket.system,
        }
    }
}

impl Default for BucketConfig {
    fn default() -> Self {
        Self {
            thumbnails: "tortoaster-thumbnails".to_owned(),
            content: "tortoaster-content".to_owned(),
            system: "tortoaster-system".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OidcConfig {
    pub client_id: String,
    #[serde(default)]
    pub client_secret: Option<String>,
    pub issuer_url: String,
    pub redirect_url: String,
}

#[serde_inline_default]
#[derive(Debug, Deserialize)]
struct CacheConfig {
    #[serde_inline_default(Ipv4Addr::LOCALHOST.to_string())]
    host: String,
    #[serde_inline_default(6379)]
    port: u16,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            host: Ipv4Addr::LOCALHOST.to_string(),
            port: 6379,
        }
    }
}
