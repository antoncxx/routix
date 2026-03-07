use crate::{
    database::{
        Database,
        models::{CertificateModel, NewCertificateModel},
    },
    repos::RepositoryError,
};

pub struct CertificatesRepository;

impl CertificatesRepository {
    pub async fn get_all(database: &Database) -> Result<Vec<CertificateModel>, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::certificates::dsl::*;
                use diesel::prelude::*;
                certificates.load::<CertificateModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn create(
        model: NewCertificateModel,
        database: &Database,
    ) -> Result<CertificateModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::certificates::dsl::*;
                use diesel::prelude::*;
                diesel::insert_into(certificates)
                    .values(&model)
                    .get_result::<CertificateModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }
}
