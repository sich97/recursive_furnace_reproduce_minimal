#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250727_130805_global_materials;
mod m20250727_130924_global_recipes;
mod m20250727_131039_create_join_table_global_recipes_and_global_materials;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250727_130805_global_materials::Migration),
            Box::new(m20250727_130924_global_recipes::Migration),
            Box::new(m20250727_131039_create_join_table_global_recipes_and_global_materials::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}