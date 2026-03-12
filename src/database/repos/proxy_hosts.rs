use crate::database::{
    Database,
    models::{NewProxyHostModel, ProxyHostModel, UpdateProxyHostModel},
    repos::RepositoryError,
};

pub struct ProxyHostsRepository;

impl ProxyHostsRepository {
    pub async fn get_all(
        database: &Database,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<ProxyHostModel>, i64), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_hosts::dsl::{id, proxy_hosts};
                use diesel::prelude::*;

                let items = proxy_hosts
                    .order(id.asc())
                    .limit(per_page)
                    .offset((page - 1) * per_page)
                    .load::<ProxyHostModel>(conn)?;

                let total = proxy_hosts.count().get_result::<i64>(conn)?;

                Ok((items, total))
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn create(
        model: NewProxyHostModel,
        database: &Database,
    ) -> Result<ProxyHostModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_hosts::dsl::proxy_hosts;
                use diesel::prelude::*;
                diesel::insert_into(proxy_hosts)
                    .values(&model)
                    .get_result::<ProxyHostModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn update(
        host_id: i32,
        model: UpdateProxyHostModel,
        database: &Database,
    ) -> Result<ProxyHostModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_hosts::dsl::{id, proxy_hosts};
                use diesel::prelude::*;
                diesel::update(proxy_hosts.filter(id.eq(host_id)))
                    .set(&model)
                    .get_result::<ProxyHostModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn update_certificate(
        host_id: i32,
        cert_name: Option<String>,
        database: &Database,
    ) -> Result<ProxyHostModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_hosts::dsl::{
                    certificate_name, id, proxy_hosts,
                };
                use diesel::prelude::*;
                diesel::update(proxy_hosts.filter(id.eq(host_id)))
                    .set(certificate_name.eq(cert_name))
                    .get_result::<ProxyHostModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn delete(host_id: i32, database: &Database) -> Result<(), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_hosts::dsl::{id, proxy_hosts};
                use diesel::prelude::*;
                diesel::delete(proxy_hosts.filter(id.eq(host_id)))
                    .execute(conn)
                    .map(|_| ())
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn fetch(
        host_id: i32,
        database: &Database,
    ) -> Result<ProxyHostModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_hosts::dsl::{id, proxy_hosts};
                use diesel::prelude::*;
                proxy_hosts
                    .filter(id.eq(host_id))
                    .first::<ProxyHostModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }
}
