use std::error::Error;

use crate::{
    context::Context,
    database::{Database, models::NewUserModel},
    repos::{RepositoryError, UsersRepository},
    roles::UserRole,
};

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub async fn initialize(context: Context) -> Result<(), Box<dyn Error>> {
    run_migrations(&context.database).await?;
    ensure_admin_exists(&context.database).await?;

    Ok(())
}

async fn run_migrations(database: &Database) -> Result<(), Box<dyn Error>> {
    log::debug!("Running migrations");

    let conn = database
        .connection()
        .await
        .map_err(RepositoryError::Connection)?;

    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .map_err(RepositoryError::Interact)?
        .map_err(RepositoryError::Other)?;

    Ok(())
}

async fn ensure_admin_exists(database: &Database) -> Result<(), Box<dyn Error>> {
    let existing = UsersRepository::find_by_username("admin", database).await?;

    if existing.is_some() {
        log::debug!("Admin user already exists, skipping creation");
        return Ok(());
    }

    log::info!("Admin user not found, creating default admin");

    let hashed = bcrypt::hash("admin", bcrypt::DEFAULT_COST)
        .map_err(|e| RepositoryError::Other(Box::new(e)))?;

    let model = NewUserModel {
        username: "admin".to_string(),
        password: hashed,
        role: UserRole::Admin.to_string(),
        scopes: vec![],
    };

    UsersRepository::create(model, database).await?;

    log::info!("Default admin user created successfully");

    Ok(())
}
