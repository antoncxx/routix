use std::error::Error;

use crate::{
    context::Context,
    database::models::NewUserModel,
    repos::{CertificatesRepository, RepositoryError, UsersRepository},
    roles::UserRole,
    tls::Certificate,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub async fn initialize(context: Context) -> Result<(), Box<dyn Error>> {
    run_migrations(&context).await?;
    ensure_admin_exists(&context).await?;
    load_certificates(&context).await?;

    Ok(())
}

async fn run_migrations(context: &Context) -> Result<(), Box<dyn Error>> {
    log::debug!("Running migrations");

    let conn = context
        .database
        .connection()
        .await
        .map_err(RepositoryError::Connection)?;

    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .map_err(RepositoryError::Interact)?
        .map_err(RepositoryError::Other)?;

    Ok(())
}

async fn ensure_admin_exists(context: &Context) -> Result<(), Box<dyn Error>> {
    let existing = UsersRepository::find_by_username("admin", &context.database).await?;

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

    UsersRepository::create(model, &context.database).await?;

    log::info!("Default admin user created successfully");

    Ok(())
}

async fn load_certificates(context: &Context) -> Result<(), Box<dyn Error>> {
    log::debug!("Loading certificates");

    let certs = CertificatesRepository::get_all(&context.database).await?;

    for cert in certs {
        if let Ok(Ok(cert_pem)) = STANDARD.decode(&cert.certificate).map(String::from_utf8)
            && let Ok(key_pem) = context
                .certificates_manager
                .decrypt_certificate_key(&cert.private_key)
            && let Ok(certificate) = Certificate::new(&cert_pem, &key_pem)
        {
            log::debug!("Loaded certificate {}", cert.name);
            let _ = certificate;
        } else {
            log::warn!("Failed to load {} certificate, ignoring ...", cert.name);
        }
    }

    Ok(())
}
