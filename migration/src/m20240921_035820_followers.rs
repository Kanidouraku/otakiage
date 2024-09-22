use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240920_132818_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Followers::Table)
                    .col(pk_auto(Followers::Id))
                    .col(integer(Followers::FolloweeId))
                    .col(integer(Followers::FollowerId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-followers-followees")
                            .from(Followers::Table, Followers::FolloweeId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-followers-followers")
                            .from(Followers::Table, Followers::FollowerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Followers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Followers {
    Table,
    Id,
    FolloweeId,
    FollowerId,
}
