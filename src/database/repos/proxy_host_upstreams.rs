use crate::database::{
    Database,
    models::{ProxyHostModel, UpstreamModel},
    repos::RepositoryError,
};

pub struct ProxyHostUpstreamsRepository;

impl ProxyHostUpstreamsRepository {
    pub async fn get_by_proxy_host(
        host_id: i32,
        database: &Database,
    ) -> Result<Vec<UpstreamModel>, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_host_upstreams::dsl::{
                    proxy_host_id, proxy_host_upstreams,
                };
                use crate::database::schema::upstreams::dsl::upstreams;
                use diesel::prelude::*;

                proxy_host_upstreams
                    .filter(proxy_host_id.eq(host_id))
                    .inner_join(upstreams)
                    .select(UpstreamModel::as_select())
                    .load::<UpstreamModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn get_all_with_upstreams(
        database: &Database,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<(ProxyHostModel, Vec<UpstreamModel>)>, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_host_upstreams::dsl::{
                    proxy_host_id, proxy_host_upstreams,
                };
                use crate::database::schema::proxy_hosts::dsl::{id, proxy_hosts};
                use crate::database::schema::upstreams::dsl::upstreams;
                use diesel::prelude::*;

                let hosts = proxy_hosts
                    .order(id.asc())
                    .limit(per_page)
                    .offset((page - 1) * per_page)
                    .load::<ProxyHostModel>(conn)?;

                let host_ids: Vec<i32> = hosts.iter().map(|h| h.id).collect();

                let pairs = proxy_host_upstreams
                    .filter(proxy_host_id.eq_any(&host_ids))
                    .inner_join(upstreams)
                    .select((proxy_host_id, UpstreamModel::as_select()))
                    .load::<(i32, UpstreamModel)>(conn)?;

                let result = hosts
                    .into_iter()
                    .map(|host| {
                        let host_upstreams = pairs
                            .iter()
                            .filter(|(pid, _)| *pid == host.id)
                            .map(|(_, u)| u.clone())
                            .collect::<Vec<UpstreamModel>>();
                        (host, host_upstreams)
                    })
                    .collect();

                Ok(result)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }
}
