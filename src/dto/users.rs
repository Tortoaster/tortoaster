use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub is_admin: bool,
}
