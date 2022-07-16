use axum::extract::{FromRequest, RequestParts};
use axum::response::Redirect;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::PrivateCookieJar;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserId(Uuid);

const SESSION: &str = "session";

#[async_trait::async_trait]
impl<B> FromRequest<B> for UserId
where
    B: Send,
{
    type Rejection =
        Result<(PrivateCookieJar, Redirect), <PrivateCookieJar as FromRequest<B>>::Rejection>;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let jar = PrivateCookieJar::from_request(req).await.map_err(Err)?;
        match jar.get(SESSION) {
            None => {
                let id = UserId(Uuid::new_v4());
                Err(Ok((
                    jar.add(Cookie::new(SESSION, id.0.to_string())),
                    Redirect::to(&req.uri().to_string()),
                )))
            }
            Some(cookie) => match cookie.value().parse::<Uuid>() {
                Ok(uuid) => Ok(UserId(uuid)),
                Err(_) => {
                    let id = UserId(Uuid::new_v4());
                    Err(Ok((
                        jar.add(Cookie::new(SESSION, id.0.to_string())),
                        Redirect::to(&req.uri().to_string()),
                    )))
                }
            },
        }
    }
}
