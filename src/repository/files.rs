use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    str::FromStr,
};

use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use thiserror::Error;
use tower_sessions_redis_store::fred::{
    clients::RedisPool, interfaces::KeysInterface, prelude::Expiration,
};
use tracing::{error, trace};
use validator::Validate;

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

    pub async fn store<T>(&self, id: impl Into<String>, file: AppFile<T>) -> AppResult<()>
    where
        T: Deref,
        T::Target: Length,
        SdkBody: From<T>,
    {
        self.client
            .put_object()
            .bucket(file.bucket.to_string())
            .key(id)
            .content_type(file.content_type.to_string())
            .content_length(file.content.len() as i64)
            .body(ByteStream::new(SdkBody::from(file.content)))
            .send()
            .await?;

        Ok(())
    }

    pub async fn store_markdown(
        &self,
        id: impl Into<String>,
        bucket: AppBucket,
        content: &str,
    ) -> AppResult<()> {
        let file = AppFile::new_markdown(content, bucket);
        let id = id.into();

        self.store(&id, file).await?;

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

#[derive(Debug, Validate)]
pub struct AppFile<T> {
    content: T,
    bucket: AppBucket,
    content_type: ContentType,
}

impl<T> AppFile<T> {
    pub fn new(content: T, bucket: AppBucket, content_type: ContentType) -> Self {
        Self {
            content,
            bucket,
            content_type,
        }
    }
}

impl<'a> AppFile<&'a str> {
    fn new_markdown(content: &'a str, bucket: AppBucket) -> Self {
        Self {
            content,
            bucket,
            content_type: ContentType::TextMarkdown,
        }
    }
}

#[derive(Debug)]
pub enum ContentType {
    TextMarkdown,
    Image(ImageType),
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::TextMarkdown => write!(f, "text/markdown"),
            ContentType::Image(ty) => write!(f, "image/{ty}"),
        }
    }
}

impl FromStr for ContentType {
    type Err = UnsupportedContentType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let (begin, end) = s.split_once('/').ok_or(UnsupportedContentType)?;
        match begin {
            "text" => match end {
                "markdown" => Ok(ContentType::TextMarkdown),
                _ => Err(UnsupportedContentType),
            },
            "image" => Ok(ContentType::Image(ImageType::from_str(end)?)),
            _ => Err(UnsupportedContentType),
        }
    }
}

#[derive(Debug)]
pub enum ImageType {
    Png,
    Jpg,
    Jpeg,
    Gif,
    Webp,
    Svg,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageType::Png => write!(f, "png"),
            ImageType::Jpg => write!(f, "jpg"),
            ImageType::Jpeg => write!(f, "jpeg"),
            ImageType::Gif => write!(f, "gif"),
            ImageType::Webp => write!(f, "webp"),
            ImageType::Svg => write!(f, "svg"),
        }
    }
}

impl FromStr for ImageType {
    type Err = UnsupportedContentType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "png" => Ok(Self::Png),
            "jpg" => Ok(Self::Jpg),
            "jpeg" => Ok(Self::Jpeg),
            "gif" => Ok(Self::Gif),
            "webp" => Ok(Self::Webp),
            "svg" => Ok(Self::Svg),
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
