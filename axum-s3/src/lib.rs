use std::{
    convert::Infallible,
    error::Error,
    fmt::{Display, Formatter},
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use aws_sdk_s3::{primitives::ByteStream, Client};
use axum_core::{
    body::Body,
    extract::Request,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use futures_core::Stream;
use http::Method;
use tower_service::Service;
use tracing::error;

#[derive(Clone, Debug)]
pub struct ServeBucket {
    client: Arc<Client>,
    bucket: String,
}

impl ServeBucket {
    pub fn new(client: impl Into<Arc<Client>>, bucket: impl Into<String>) -> Self {
        Self {
            client: client.into(),
            bucket: bucket.into(),
        }
    }
}

impl Service<Request> for ServeBucket {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        Box::pin(handle_request(
            req,
            self.client.clone(),
            self.bucket.clone(),
        ))
    }
}

async fn handle_request(
    req: Request,
    client: Arc<Client>,
    bucket: String,
) -> Result<Response, Infallible> {
    Ok(try_handle_request(req, client, bucket)
        .await
        .into_response())
}

async fn try_handle_request(
    req: Request,
    client: Arc<Client>,
    bucket: String,
) -> Result<Response, S3Error> {
    if *req.method() != Method::GET {
        return Err(S3Error::WrongMethod);
    }

    let key = percent_encoding::percent_decode_str(
        req.uri().path().strip_prefix('/').ok_or(S3Error::NoKey)?,
    )
    .decode_utf8()?
    .into();

    try_retrieve_from_bucket(client, bucket, key).await
}

async fn try_retrieve_from_bucket(
    client: Arc<Client>,
    bucket: String,
    key: String,
) -> Result<Response, S3Error> {
    let output = client.get_object().bucket(bucket).key(&key).send().await?;

    let mut builder = Response::builder();

    if let Some(metadata) = output.metadata {
        for (key, value) in metadata {
            builder = builder.header(key, value);
        }
    }

    let response = builder.body(Body::from_stream(ActualByteStream(output.body)))?;

    Ok(response)
}

#[allow(dead_code)]
#[derive(Debug)]
enum S3Error {
    Aws(aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::get_object::GetObjectError>),
    Http(http::Error),
    Encoding(std::str::Utf8Error),
    NoKey,
    WrongMethod,
}

impl S3Error {
    fn status_code(&self) -> u16 {
        404
    }
}

impl IntoResponse for S3Error {
    fn into_response(self) -> Response {
        error!("{self:?}");

        Response::builder()
            .status(self.status_code())
            .body(Body::new(self.to_string()))
            .unwrap()
    }
}

impl Display for S3Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resource not found")
    }
}

impl Error for S3Error {}

impl From<aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::get_object::GetObjectError>>
    for S3Error
{
    fn from(
        value: aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::get_object::GetObjectError>,
    ) -> Self {
        S3Error::Aws(value)
    }
}

impl From<http::Error> for S3Error {
    fn from(value: http::Error) -> Self {
        S3Error::Http(value)
    }
}

impl From<std::str::Utf8Error> for S3Error {
    fn from(value: std::str::Utf8Error) -> Self {
        S3Error::Encoding(value)
    }
}

/// Because `aws_sdk_s3`'s [`ByteStream`] doesn't actually implement [`Stream`].
struct ActualByteStream(ByteStream);

impl Stream for ActualByteStream {
    type Item = Result<Bytes, aws_smithy_types::byte_stream::error::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unsafe { self.map_unchecked_mut(|stream| &mut stream.0) }.poll_next(cx)
    }
}
