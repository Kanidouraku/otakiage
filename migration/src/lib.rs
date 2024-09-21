#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20240920_132818_users;
mod m20240920_153342_posts;
mod m20240921_035820_followers;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240920_132818_users::Migration),
            Box::new(m20240920_153342_posts::Migration),
            Box::new(m20240921_035820_followers::Migration),
        ]
    }
}