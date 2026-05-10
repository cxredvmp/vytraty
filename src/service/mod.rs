mod auth;
mod category;
mod health;
mod record;
mod user;

pub use auth::Service as Auth;
pub use category::Service as Category;
pub use health::Service as Health;
pub use record::Service as Record;
pub use user::Service as User;
