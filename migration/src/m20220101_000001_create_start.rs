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
                    .col(ColumnDef::new(Users::Uid).string().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).date_time().not_null())
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
                                .default(false),
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
                            .col(ColumnDef::new(Commits::Description).string())
                            .col(ColumnDef::new(Commits::CreatedBy).integer().not_null())
                            .col(
                                ColumnDef::new(Commits::CreatedAt).date_time().not_null(),
                            )
                            .foreign_key(
                                ForeignKey::create()
                                    .name("FK_COMMIT_CREATED_BY")
                                    .from(Commits::Table, Commits::CreatedBy)
                                    .to(Users::Table, Users::Id)
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
                    .col(ColumnDef::new(Files::Type).string())
                    .col(ColumnDef::new(Files::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Files::CreatedBy).integer().not_null())
                    .col(ColumnDef::new(Files::CheckedOutStatus).boolean().not_null())
                    .col(ColumnDef::new(Files::CheckedOutBy).integer().not_null())
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
                    .col(ColumnDef::new(Versions::VersionedAt).date_time().not_null())
                    .col(ColumnDef::new(Versions::VersionedBy).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_USER_VERSION_VERSIONED_BY")
                            .from(Versions::Table, Versions::VersionedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserProjects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserProjects::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserProjects::UserId).integer().not_null())
                    .col(ColumnDef::new(UserProjects::ProjectId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_USER_PROJECTS_USER_ID")
                            .from(UserProjects::Table, UserProjects::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_USER_PROJECTS_PROJECT_ID")
                            .from(UserProjects::Table, UserProjects::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

            manager
                .create_table(
                    Table::create()
                        .table(ProjectFiles::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(ProjectFiles::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(ProjectFiles::FileId).integer().not_null())
                        .col(ColumnDef::new(ProjectFiles::ProjectId).integer().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("FK_PROJECT_FILES_PROJECT_ID")
                                .from(ProjectFiles::Table, ProjectFiles::ProjectId)
                                .to(Projects::Table, Projects::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .foreign_key(
                            ForeignKey::create()
                                .name("FK_PROJECT_FILES_FILE_ID")
                                .from(ProjectFiles::Table, ProjectFiles::FileId)
                                .to(Files::Table, Files::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
            )
            .await?;

            manager
                .create_table(
                    Table::create()
                        .table(ProjectCommits::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(ProjectCommits::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(ProjectCommits::CommitId).integer().not_null())
                        .col(ColumnDef::new(ProjectCommits::ProjectId).integer().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("FK_PROJECT_COMMITS_PROJECT_ID")
                                .from(ProjectCommits::Table, ProjectCommits::ProjectId)
                                .to(Projects::Table, Projects::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .foreign_key(
                            ForeignKey::create()
                                .name("FK_PROJECT_COMMITS_COMMIT_ID")
                                .from(ProjectCommits::Table, ProjectCommits::CommitId)
                                .to(Commits::Table, Commits::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
            )
            .await?;

            manager
                .create_table(
                    Table::create()
                        .table(CommitVersions::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(CommitVersions::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(CommitVersions::CommitId).integer().not_null())
                        .col(ColumnDef::new(CommitVersions::VersionId).integer().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("FK_COMMITS_VERSION_ID")
                                .from(CommitVersions::Table, CommitVersions::VersionId)
                                .to(Versions::Table, Versions::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .foreign_key(
                            ForeignKey::create()
                                .name("FK_COMMITS_COMMIT_ID")
                                .from(CommitVersions::Table, CommitVersions::CommitId)
                                .to(Commits::Table, Commits::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
            )
            .await?;

            manager
                .create_table(
                    Table::create()
                        .table(FileVersions::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(FileVersions::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(FileVersions::FileId).integer().not_null())
                        .col(ColumnDef::new(FileVersions::VersionId).integer().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("FK_FILE_VERSIONS_FILE_ID")
                                .from(FileVersions::Table, FileVersions::VersionId)
                                .to(Files::Table, Files::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .foreign_key(
                            ForeignKey::create()
                                .name("FK_FILE_VERSION_VERSION_ID")
                                .from(FileVersions::Table, FileVersions::FileId)
                                .to(Versions::Table, Versions::Id)
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserProjects::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ProjectFiles::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FileVersions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CommitVersions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Files::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Versions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Commits::Table).to_owned())
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

/// Mapping of Users to Projects
#[derive(Iden)]
enum UserProjects {
    Table,
    Id,
    UserId,
    ProjectId,
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

/// A mapping if Projects to Commits
#[derive(Iden)]
enum ProjectCommits {
    Table,
    Id,
    ProjectId,
    CommitId,
}

/// A mapping of Projects to Files
#[derive(Iden)]
enum ProjectFiles {
    Table,
    Id,
    ProjectId,
    FileId,
}

/// A set of commits. Each commit contains a list of versions of files changed.
#[derive(Iden)]
enum Commits {
    Table,
    Id,
    Description,
    CreatedAt,
    CreatedBy
}

/// A mapping of Commits to Versions of files
#[derive(Iden)]
enum CommitVersions {
    Table,
    Id,
    CommitId,
    VersionId,
}


/// A set of versions of files. Keyed differently than everything else, as they're keyed based on file hash.
#[derive(Iden)]
enum Versions {
    Table,
    Id,
    /// Path to the object in object storage
    ObjectPath,
    VersionedAt,
    VersionedBy,
    VersionNumber,
}


/// A mapping of Versions to Files
#[derive(Iden)]
enum FileVersions {
    Table,
    Id,
    FileId,
    VersionId,
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
    Type,
    CheckedOutStatus,
    CheckedOutBy,
}

