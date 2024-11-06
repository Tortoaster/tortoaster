use std::{
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::OnceLock,
};

use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::config::Credentials;
use config::Config;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use sqlx::postgres::PgConnectOptions;
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

    pub fn pg_connect_options(&self) -> PgConnectOptions {
        let mut options = self
            .database
            .url
            .parse::<PgConnectOptions>()
            .expect("invalid database url");

        if let Some(password) = self.database.password.as_deref() {
            options = options.password(password);
        }

        options
    }

    pub async fn s3_config(&self) -> aws_sdk_s3::Config {
        let sdk_config = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .region(Region::new(self.s3.region.clone()))
            .endpoint_url(&self.s3.endpoint_url)
            .credentials_provider(Credentials::new(
                &self.s3.access_key_id,
                &self.s3.secret_access_key,
                self.s3.session_token.clone(),
                None,
                "tortoaster-credential-provider",
            ))
            .load()
            .await;

        aws_sdk_s3::config::Builder::from(&sdk_config)
            // Required by MinIO
            .force_path_style(true)
            .build()
    }

    pub fn s3_bucket_name(&self) -> &str {
        &self.s3.bucket_name
    }

    pub fn s3_bucket_url(&self) -> &str {
        &self.s3.bucket_url
    }
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    password: Option<String>,
}

#[serde_inline_default]
#[derive(Debug, Deserialize)]
struct S3Config {
    region: String,
    endpoint_url: String,
    access_key_id: String,
    secret_access_key: String,
    #[serde(default)]
    session_token: Option<String>,
    #[serde_inline_default("tortoaster".to_owned())]
    bucket_name: String,
    bucket_url: String,
}

#[derive(Debug, Deserialize)]
pub struct OidcConfig {
    pub client_id: String,
    #[serde(default)]
    pub client_secret: Option<String>,
    pub issuer_url: String,
    pub redirect_url: String,
}
