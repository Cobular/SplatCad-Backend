use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Uid).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Projects::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Projects::Name).string().not_null())
                    .col(ColumnDef::new(Projects::Description).string())
                    .col(ColumnDef::new(Projects::CreatedBy).integer().not_null())
                    .col(
                        ColumnDef::new(Projects::EnforceCheckouts)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_USER_PROJECT_CREATED_BY")
                            .from(Projects::Table, Projects::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Commits::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Commits::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Commits::ProjectId).integer().not_null())
                    .col(ColumnDef::new(Commits::Description).string())
                    .col(ColumnDef::new(Commits::CreatedBy).integer().not_null())
                    .col(ColumnDef::new(Commits::CommitNumber).integer().not_null())
                    .col(
                        ColumnDef::new(Commits::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_COMMIT_CREATED_BY")
                            .from(Commits::Table, Commits::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_COMMIT_PROJECT")
                            .from(Commits::Table, Commits::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Files::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Files::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Files::Name).string().not_null())
                    .col(ColumnDef::new(Files::FileType).string())
                    .col(
                        ColumnDef::new(Files::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Files::CreatedBy).integer().not_null())
                    .col(ColumnDef::new(Files::CheckedOutStatus).boolean().not_null())
                    .col(ColumnDef::new(Files::CheckedOutBy).integer())
                    .col(ColumnDef::new(Files::ProjectId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_USER_FILES_CREATED_BY")
                            .from(Files::Table, Files::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_USER_FILES_CHECKED_OUT_BY")
                            .from(Files::Table, Files::CheckedOutBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_FILE_PROJECT")
                            .from(Files::Table, Files::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Versions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Versions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Versions::ObjectPath).string().not_null())
                    .col(ColumnDef::new(Versions::VersionNumber).integer().not_null())
                    .col(
                        ColumnDef::new(Versions::VersionedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Versions::VersionedBy).integer().not_null())
                    .col(ColumnDef::new(Versions::CommitId).integer().not_null())
                    .col(ColumnDef::new(Versions::FileId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_USER_VERSION_VERSIONED_BY")
                            .from(Versions::Table, Versions::VersionedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_VERSION_COMMIT")
                            .from(Versions::Table, Versions::CommitId)
                            .to(Commits::Table, Commits::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_VERSION_FILE")
                            .from(Versions::Table, Versions::FileId)
                            .to(Files::Table, Files::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Versions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Files::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Commits::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// User metadata - logins in auth0
#[derive(Iden)]
enum Users {
    Table,
    Id,
    Uid,
    CreatedAt,
}

/// Projects - A collection of commits, which each contain versions which contain files.
/// Also contains a list of files for easier lookup.
#[derive(Iden)]
enum Projects {
    Table,
    Id,
    Name,
    Description,
    EnforceCheckouts,
    CreatedBy,
}

/// A set of commits. Each commit contains a list of versions of files changed.
#[derive(Iden)]
enum Commits {
    Table,
    Id,
    Description,
    CreatedAt,
    CreatedBy,
    ProjectId,
    CommitNumber
}

/// A set of versions of files.
#[derive(Iden)]
enum Versions {
    Table,
    Id,
    /// Path to the object in object storage
    ObjectPath,
    VersionedAt,
    VersionedBy,
    VersionNumber,

    FileId,
    CommitId,
}

/// Individual files. Each file has multiple versions and through that belongs to many commits.
/// Each file also belongs to a project.
#[derive(Iden)]
enum Files {
    Table,
    Id,
    Name,
    CreatedBy,
    CreatedAt,
    FileType,
    CheckedOutStatus,
    CheckedOutBy,

    ProjectId,
}
