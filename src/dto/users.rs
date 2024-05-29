use uuid::Uuid;

use crate::model::user_entity;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub is_admin: bool,
}

impl User {
    pub fn deleted() -> Self {
        User {
            id: Uuid::nil(),
            name: "[deleted user]".to_owned(),
            is_admin: false,
        }
    }
}

impl TryFrom<user_entity::Model> for User {
    type Error = uuid::Error;

    fn try_from(value: user_entity::Model) -> Result<Self, Self::Error> {
        Ok(User {
            id: value.id.parse()?,
            name: value
                .username
                .unwrap_or_else(|| "[unknown user]".to_owned()),
            // TODO: Retrieve admin status
            is_admin: false,
        })
    }
}
