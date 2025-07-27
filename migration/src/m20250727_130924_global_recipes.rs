use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "global_recipes",
            &[
            
            ("id", ColType::PkAuto),
            
            ("machine_fuel_consumption", ColType::DoubleNull),
            ("hash", ColType::TextUniq),
            ],
            &[
            ("user", "created_by"),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "global_recipes").await
    }
}
