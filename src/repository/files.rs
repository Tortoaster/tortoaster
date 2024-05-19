use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    str::FromStr,
};

use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

use crate::{config::AppBucket, error::AppResult};

#[derive(Clone, Debug)]
pub struct FileRepository {
    client: aws_sdk_s3::Client,
}

impl FileRepository {
    pub fn new(client: aws_sdk_s3::Client) -> Self {
        Self { client }
    }

    pub async fn store<T>(&self, id: Uuid, file: AppFile<T>) -> AppResult<()>
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
    pub fn new_markdown(content: &'a str) -> Self {
        Self {
            content,
            bucket: AppBucket::Content,
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
