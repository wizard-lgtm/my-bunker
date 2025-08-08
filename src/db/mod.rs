pub mod connect;
pub mod users;

pub struct Db {
    pub user_repo: users::UserRepo,
}