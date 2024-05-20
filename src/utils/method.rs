use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Method {
    Post,
    Patch,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Post => write!(f, "hx-post"),
            Method::Patch => write!(f, "hx-patch"),
        }
    }
}
