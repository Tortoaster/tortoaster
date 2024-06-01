pub type UserId = String;

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub name: Option<String>,
    pub is_admin: bool,
}
