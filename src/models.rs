pub struct Database {
    pub users: Vec<User>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub password: String,
}
