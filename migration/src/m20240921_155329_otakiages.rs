use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Otakiages::Table)
                    .col(pk_auto(Otakiages::Id))
                    .col(integer(Otakiages::PostId))
                    .col(integer(Otakiages::Count))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-otakiages-posts")
                            .from(Otakiages::Table, Otakiages::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Otakiages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Otakiages {
    Table,
    Id,
    PostId,
    Count,
    
}


#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}
