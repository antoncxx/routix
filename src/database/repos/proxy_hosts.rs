use crate::database::{
    Database,
    models::{NewProxyHostModel, ProxyHostModel, ProxyHostUpstreamModel, UpdateProxyHostModel},
    repos::RepositoryError,
};

pub struct ProxyHostsRepository;

impl ProxyHostsRepository {
    pub async fn create(
        model: NewProxyHostModel,
        upstream_ids: Vec<i32>,
        database: &Database,
    ) -> Result<ProxyHostModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_host_upstreams::dsl::proxy_host_upstreams;
                use crate::database::schema::proxy_hosts::dsl::proxy_hosts;
                use diesel::prelude::*;

                conn.transaction(|conn| {
                    let host = diesel::insert_into(proxy_hosts)
                        .values(&model)
                        .get_result::<ProxyHostModel>(conn)?;

                    let links: Vec<ProxyHostUpstreamModel> = upstream_ids
                        .into_iter()
                        .map(|upstream_id| ProxyHostUpstreamModel {
                            proxy_host_id: host.id,
                            upstream_id,
                        })
                        .collect();

                    diesel::insert_into(proxy_host_upstreams)
                        .values(&links)
                        .execute(conn)?;

                    Ok(host)
                })
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

    pub async fn update_access_list(
        host_id: i32,
        list_id: Option<i32>,
        database: &Database,
    ) -> Result<ProxyHostModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_hosts::dsl::{access_list_id, id, proxy_hosts};
                use diesel::prelude::*;
                diesel::update(proxy_hosts.filter(id.eq(host_id)))
                    .set(access_list_id.eq(list_id))
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

    pub async fn count(database: &Database) -> Result<i64, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_hosts::dsl::proxy_hosts;
                use diesel::prelude::*;
                proxy_hosts.count().get_result::<i64>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }
}
