use sea_orm_migration::{prelude::*, sea_orm::{ActiveModelTrait, TransactionTrait}};
use entity::{users, projects, commits, files, versions};
use sea_orm_migration::sea_orm::{entity::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let txn = db.begin().await?;

        println!("Seeding Users");
        // Create Users
        users::ActiveModel {
            uid: Set("admin".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            id: Set(1)
        }.insert(&txn).await?;
        users::ActiveModel {
            uid: Set("user".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            id: Set(2)
        }.insert(&txn).await?;

        println!("Seeding Projects");
        // Create Projects
        projects::ActiveModel {
            name: Set("project1-admin".to_string()),
            description: Set(Some("description1".to_string())),
            created_by: Set(1),
            enforce_checkouts: Set(true),
            id: Set(1)
        }.insert(&txn).await?;
        projects::ActiveModel {
            name: Set("project1-user".to_string()),
            description: Set(Some("description1".to_string())),
            created_by: Set(2),
            enforce_checkouts: Set(false),
            id: Set(2)
        }.insert(&txn).await?;

        println!("Seeding Commits");
        // Create some commits for those projects
        // Project 1
        commits::ActiveModel {
            id: Set(1),
            project_id: Set(1),
            created_by: Set(1),
            created_at: Set(chrono::Utc::now().into()),
            description: Set(Some("commit1".to_string())),
            commit_number: Set(1)
        }.insert(&txn).await?;
        commits::ActiveModel {
            id: Set(2),
            project_id: Set(1),
            created_by: Set(1),
            created_at: Set(chrono::Utc::now().into()),
            description: Set(Some("commit2".to_string())),
            commit_number: Set(2)
        }.insert(&txn).await?;
        // Project 2
        commits::ActiveModel {
            id: Set(3),
            project_id: Set(2),
            created_by: Set(2),
            created_at: Set(chrono::Utc::now().into()),
            description: Set(Some("commit3".to_string())),
            commit_number: Set(1)
        }.insert(&txn).await?;
        commits::ActiveModel {
            id: Set(4),
            project_id: Set(2),
            created_by: Set(2),
            created_at: Set(chrono::Utc::now().into()),
            description: Set(Some("commit4".to_string())),
            commit_number: Set(2)
        }.insert(&txn).await?;

        println!("Seeding Files");
        // Create some files for those projects
        // Project 1
        files::ActiveModel {
            id: Set(1),
            project_id: Set(1),
            name: Set("file1".to_string()),
            checked_out_by: Set(None),
            created_by: Set(1),
            created_at: Set(chrono::Utc::now().into()),
            checked_out_status: Set(false),
            file_type: Set(Some("stl".to_string())),
        }.insert(&txn).await?;
        files::ActiveModel {
            id: Set(2),
            project_id: Set(1),
            name: Set("file2".to_string()),
            checked_out_by: Set(Some(1)),
            created_by: Set(1),
            created_at: Set(chrono::Utc::now().into()),
            checked_out_status: Set(true),
            file_type: Set(Some("step".to_string())),
        }.insert(&txn).await?;
        files::ActiveModel {
            id: Set(3),
            project_id: Set(1),
            name: Set("file1".to_string()),
            checked_out_by: Set(None),
            created_by: Set(1),
            created_at: Set(chrono::Utc::now().into()),
            checked_out_status: Set(false),
            file_type: Set(Some("jpg".to_string())),
        }.insert(&txn).await?;
        // Project2
        files::ActiveModel {
            id: Set(4),
            project_id: Set(2),
            name: Set("file21".to_string()),
            checked_out_by: Set(None),
            created_by: Set(2),
            created_at: Set(chrono::Utc::now().into()),
            checked_out_status: Set(false),
            file_type: Set(Some("stl".to_string())),
        }.insert(&txn).await?;
        files::ActiveModel {
            id: Set(5),
            project_id: Set(2),
            name: Set("file22".to_string()),
            checked_out_by: Set(Some(2)),
            created_by: Set(2),
            created_at: Set(chrono::Utc::now().into()),
            checked_out_status: Set(true),
            file_type: Set(Some("step".to_string())),
        }.insert(&txn).await?;
        files::ActiveModel {
            id: Set(6),
            project_id: Set(2),
            name: Set("file21".to_string()),
            checked_out_by: Set(None),
            created_by: Set(2),
            created_at: Set(chrono::Utc::now().into()),
            checked_out_status: Set(false),
            file_type: Set(Some("jpg".to_string())),
        }.insert(&txn).await?;

        println!("Seeding Versions");
        // Create some versions associated to the files
        // Project 1
        //  Commit 1
        versions::ActiveModel {
            id: Set(1),
            file_id: Set(1),
            object_path: Set("https://object.com/1/1".to_string()),
            versioned_at: Set(chrono::Utc::now().into()),
            versioned_by: Set(1),
            commit_id: Set(1),
            version_number: Set(1)
        }.insert(&txn).await?;
        versions::ActiveModel {
            id: Set(2),
            file_id: Set(2),
            object_path: Set("https://object.com/2/1".to_string()),
            versioned_at: Set(chrono::Utc::now().into()),
            versioned_by: Set(1),
            commit_id: Set(1),
            version_number: Set(1)
        }.insert(&txn).await?;
        //  Commit 2
        versions::ActiveModel {
            id: Set(3),
            file_id: Set(2),
            object_path: Set("https://object.com/2/2".to_string()),
            versioned_at: Set(chrono::Utc::now().into()),
            versioned_by: Set(1),
            commit_id: Set(2),
            version_number: Set(2)
        }.insert(&txn).await?;
        versions::ActiveModel {
            id: Set(4),
            file_id: Set(3),
            object_path: Set("https://object.com/3/1".to_string()),
            versioned_at: Set(chrono::Utc::now().into()),
            versioned_by: Set(1),
            commit_id: Set(2),
            version_number: Set(1)
        }.insert(&txn).await?;
        // Project 2
        //  Commit 1
        versions::ActiveModel {
            id: Set(5),
            file_id: Set(4),
            object_path: Set("https://object.com/4/1".to_string()),
            versioned_at: Set(chrono::Utc::now().into()),
            versioned_by: Set(2),
            commit_id: Set(3),
            version_number: Set(1)
        }.insert(&txn).await?;
        versions::ActiveModel {
            id: Set(6),
            file_id: Set(5),
            object_path: Set("https://object.com/5/1".to_string()),
            versioned_at: Set(chrono::Utc::now().into()),
            versioned_by: Set(2),
            commit_id: Set(3),
            version_number: Set(1)
        }.insert(&txn).await?;
        //  Commit 2
        versions::ActiveModel {
            id: Set(7),
            file_id: Set(4),
            object_path: Set("https://object.com/2/2".to_string()),
            versioned_at: Set(chrono::Utc::now().into()),
            versioned_by: Set(2),
            commit_id: Set(4),
            version_number: Set(2)
        }.insert(&txn).await?;
        versions::ActiveModel {
            id: Set(8),
            file_id: Set(6),
            object_path: Set("https://object.com/6/1".to_string()),
            versioned_at: Set(chrono::Utc::now().into()),
            versioned_by: Set(2),
            commit_id: Set(4),
            version_number: Set(1)
        }.insert(&txn).await?;

        
        txn.commit().await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        versions::Entity::delete_by_id(1).exec(db).await?;
        versions::Entity::delete_by_id(2).exec(db).await?;
        versions::Entity::delete_by_id(3).exec(db).await?;
        versions::Entity::delete_by_id(4).exec(db).await?;
        versions::Entity::delete_by_id(5).exec(db).await?;
        versions::Entity::delete_by_id(6).exec(db).await?;
        versions::Entity::delete_by_id(7).exec(db).await?;
        versions::Entity::delete_by_id(8).exec(db).await?;

        files::Entity::delete_by_id(1).exec(db).await?;
        files::Entity::delete_by_id(2).exec(db).await?;
        files::Entity::delete_by_id(3).exec(db).await?;
        files::Entity::delete_by_id(4).exec(db).await?;
        files::Entity::delete_by_id(5).exec(db).await?;
        files::Entity::delete_by_id(6).exec(db).await?;

        commits::Entity::delete_by_id(1).exec(db).await?;
        commits::Entity::delete_by_id(2).exec(db).await?;
        commits::Entity::delete_by_id(3).exec(db).await?;
        commits::Entity::delete_by_id(4).exec(db).await?;

        projects::Entity::delete_by_id(2).exec(db).await?;
        projects::Entity::delete_by_id(1).exec(db).await?;

        users::Entity::delete_by_id(2).exec(db).await?;
        users::Entity::delete_by_id(1).exec(db).await?;

        Ok(())
    }
}
