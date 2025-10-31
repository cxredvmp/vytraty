pub use sea_orm_migration::prelude::*;

mod m20251027_233718_create_users_table;
mod m20251027_234532_create_categories_table;
mod m20251027_234708_create_records_table;
mod m20251028_001126_create_currency_table;
mod m20251028_011150_add_default_currency_code_to_user_table;
mod m20251028_011557_add_currency_code_to_record_table;
mod m20251028_012312_seed_currency_table;
mod m20251115_161146_add_password_hash_to_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251027_233718_create_users_table::Migration),
            Box::new(m20251027_234532_create_categories_table::Migration),
            Box::new(m20251027_234708_create_records_table::Migration),
            Box::new(m20251028_001126_create_currency_table::Migration),
            Box::new(m20251028_011150_add_default_currency_code_to_user_table::Migration),
            Box::new(m20251028_011557_add_currency_code_to_record_table::Migration),
            Box::new(m20251028_012312_seed_currency_table::Migration),
            Box::new(m20251115_161146_add_password_hash_to_user_table::Migration),
        ]
    }
}
