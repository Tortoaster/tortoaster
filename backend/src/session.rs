use std::fmt::{Display, Formatter};

use axum::extract::{FromRequest, RequestParts};
use axum::response::Redirect;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::SignedCookieJar;
use names::Generator;

const SESSION: &str = "session";

#[derive(Debug)]
pub struct User(String);

impl User {
    pub fn generate() -> Self {
        let mut generator = Generator::default();
        User(generator.next().unwrap())
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection =
        Result<(SignedCookieJar, Redirect), <SignedCookieJar as FromRequest<B>>::Rejection>;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let jar = SignedCookieJar::from_request(req).await.map_err(Err)?;
        match jar.get(SESSION) {
            None => Err(Ok((
                jar.add(Cookie::new(SESSION, User::generate().to_string())),
                Redirect::to(&req.uri().to_string()),
            ))),
            Some(cookie) => Ok(User(cookie.value().to_owned())),
        }
    }
}
