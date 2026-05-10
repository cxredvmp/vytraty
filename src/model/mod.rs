pub mod auth;
pub mod category;
pub mod currency;
pub mod health;
pub mod record;
pub mod user;

pub fn models() -> toasty::ModelSet {
    toasty::models!(
        user::User,
        category::Category,
        currency::Currency,
        record::Record
    )
}
