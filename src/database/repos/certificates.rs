use crate::database::{
    Database,
    models::{CertificateModel, NewCertificateModel},
    repos::RepositoryError,
};

pub struct CertificatesRepository;

impl CertificatesRepository {
    pub async fn get_all(
        database: &Database,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<CertificateModel>, i64), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::certificates::dsl::{certificates, id};
                use diesel::prelude::*;

                let items = certificates
                    .order(id.asc())
                    .limit(per_page)
                    .offset((page - 1) * per_page)
                    .load::<CertificateModel>(conn)?;

                let total = certificates.count().get_result::<i64>(conn)?;

                Ok((items, total))
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn fetch(
        cert_id: i32,
        database: &Database,
    ) -> Result<CertificateModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::certificates::dsl::{certificates, id};
                use diesel::prelude::*;
                certificates
                    .filter(id.eq(cert_id))
                    .first::<CertificateModel>(conn)
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
                use crate::database::schema::certificates::dsl::certificates;
                use diesel::prelude::*;
                diesel::insert_into(certificates)
                    .values(&model)
                    .get_result::<CertificateModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn delete(cert_id: i32, database: &Database) -> Result<(), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::certificates::dsl::{certificates, id};
                use diesel::prelude::*;
                diesel::delete(certificates.filter(id.eq(cert_id)))
                    .execute(conn)
                    .map(|_| ())
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }
}
