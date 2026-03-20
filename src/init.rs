use std::error::Error;

use crate::{
    context::Context,
    database::{
        models::NewUserModel,
        repos::{
            CertificatesRepository, ProxyHostUpstreamsRepository, ProxyHostsRepository,
            RepositoryError, UsersRepository,
        },
    },
    proxy::ProxyHost,
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
    load_proxy_hosts(&context).await?;

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

    let per_page = 100;
    let mut page = 1;

    loop {
        let (certs, total) =
            CertificatesRepository::get_all(&context.database, page, per_page).await?;

        for cert in certs {
            if let Ok(Ok(cert_pem)) = STANDARD.decode(&cert.certificate).map(String::from_utf8)
                && let Ok(key_pem) = context
                    .certificates_manager
                    .decrypt_certificate_key(&cert.private_key)
                && let Ok(certificate) = Certificate::new(&cert_pem, &key_pem)
            {
                log::debug!("Loaded certificate {}", cert.name);
                let _ = context
                    .certificates_manager
                    .add(&cert.name, certificate)
                    .await;
            } else {
                log::warn!("Failed to load {} certificate, ignoring ...", cert.name);
            }
        }

        let total_pages = (total + per_page - 1) / per_page;
        if page >= total_pages {
            break;
        }
        page += 1;
    }

    Ok(())
}

async fn load_proxy_hosts(context: &Context) -> Result<(), Box<dyn Error>> {
    log::debug!("Loading proxy hosts");

    let per_page = 100;
    let mut page = 1;

    loop {
        let data =
            ProxyHostUpstreamsRepository::get_all_with_upstreams(&context.database, page, per_page)
                .await?;

        let total = ProxyHostsRepository::count(&context.database).await?;

        for (proxy_host_model, upstream_models, access_list_data) in data {
            let proxy_domain = proxy_host_model.domain.clone();
            match ProxyHost::new(proxy_host_model, upstream_models, access_list_data) {
                Ok(proxy_host) => {
                    context.hosts_manager.add(proxy_host).await;
                }
                Err(_) => {
                    log::warn!("Failed to load proxy host {proxy_domain}");
                }
            }
        }

        let total_pages = (total + per_page - 1) / per_page;

        if page >= total_pages {
            break;
        }

        page += 1;
    }

    Ok(())
}
