use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    str::FromStr,
};

use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use bytes::Bytes;
use thiserror::Error;
use tower_sessions_redis_store::fred::{
    clients::RedisPool, interfaces::KeysInterface, prelude::Expiration,
};
use tracing::{error, trace};

use crate::{
    config::AppBucket,
    error::{AppError, AppResult},
};

#[derive(Clone, Debug)]
pub struct FileRepository {
    client: aws_sdk_s3::Client,
    redis_pool: RedisPool,
}

impl FileRepository {
    pub fn new(client: aws_sdk_s3::Client, redis_pool: RedisPool) -> Self {
        Self { client, redis_pool }
    }

    async fn store<T>(
        &self,
        id: impl Into<String>,
        content: T,
        bucket: AppBucket,
        content_type: impl Into<String>,
    ) -> AppResult<()>
    where
        T: Deref,
        T::Target: Length,
        SdkBody: From<T>,
    {
        self.client
            .put_object()
            .bucket(bucket.to_string())
            .key(id)
            .content_type(content_type)
            .content_length(content.len() as i64)
            .body(ByteStream::new(SdkBody::from(content)))
            .send()
            .await?;

        Ok(())
    }

    pub async fn store_image(
        &self,
        id: impl Into<String>,
        bucket: AppBucket,
        bytes: Bytes,
        content_type: ImageContentType,
    ) -> AppResult<()> {
        let id = id.into();

        self.store(&id, bytes, bucket, content_type.to_string())
            .await?;

        Ok(())
    }

    pub async fn store_markdown(
        &self,
        id: impl Into<String>,
        bucket: AppBucket,
        content: &str,
    ) -> AppResult<()> {
        let id = id.into();

        self.store(&id, content, bucket, "text/markdown").await?;

        self.store_in_cache(&id, bucket, content).await;

        Ok(())
    }

    pub async fn retrieve_markdown(
        &self,
        id: impl Into<String>,
        bucket: AppBucket,
    ) -> AppResult<String> {
        let id = id.into();

        if let Some(content) = self.retrieve_from_cache(&id, bucket).await {
            trace!("found cached entry for {}/{id}", bucket.name());
            return Ok(content);
        }
        trace!("no cached entry for {}/{id}", bucket.name());

        let content = String::from_utf8(
            self.client
                .get_object()
                .bucket(bucket.to_string())
                .key(&id)
                .send()
                .await?
                .body
                .collect()
                .await
                .map_err(|_| AppError::ObjectEncoding)?
                .to_vec(),
        )
        .map_err(|_| AppError::ObjectEncoding)?;

        self.store_in_cache(&id, bucket, &content).await;

        Ok(content)
    }

    async fn store_in_cache(&self, id: &str, bucket: AppBucket, content: &str) {
        if let Err(error) = self
            .redis_pool
            .set::<(), _, _>(
                Self::cache_key(id, bucket),
                content,
                Some(Expiration::EX(24 * 3600)),
                None,
                false,
            )
            .await
        {
            error!("failed to store in cache: {error}")
        }
    }

    async fn retrieve_from_cache(&self, id: &str, bucket: AppBucket) -> Option<String> {
        self.redis_pool
            .get::<Option<String>, _>(Self::cache_key(id, bucket))
            .await
            .unwrap_or_else(|error| {
                error!("failed to retrieve from cache: {error}");
                None
            })
    }

    fn cache_key(id: &str, bucket: AppBucket) -> String {
        format!("{}/{id}", bucket.name())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ImageContentType {
    Png,
    Jpg,
    Jpeg,
    Gif,
    Webp,
    Svg,
}

impl Display for ImageContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageContentType::Png => write!(f, "image/png"),
            ImageContentType::Jpg => write!(f, "image/jpg"),
            ImageContentType::Jpeg => write!(f, "image/jpeg"),
            ImageContentType::Gif => write!(f, "image/gif"),
            ImageContentType::Webp => write!(f, "image/webp"),
            ImageContentType::Svg => write!(f, "image/svg+xml"),
        }
    }
}

impl FromStr for ImageContentType {
    type Err = UnsupportedContentType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "image/png" => Ok(Self::Png),
            "image/jpg" => Ok(Self::Jpg),
            "image/jpeg" => Ok(Self::Jpeg),
            "image/gif" => Ok(Self::Gif),
            "image/webp" => Ok(Self::Webp),
            "image/svg+xml" => Ok(Self::Svg),
            _ => Err(UnsupportedContentType),
        }
    }
}

#[derive(Debug, Error)]
#[error("unsupported content type")]
pub struct UnsupportedContentType;

pub trait Length {
    fn len(&self) -> usize;
}

impl Length for [u8] {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Length for str {
    fn len(&self) -> usize {
        self.len()
    }
}
