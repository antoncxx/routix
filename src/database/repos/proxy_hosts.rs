use crate::database::{
    Database,
    models::{
        AccessListModel, AccessListRuleModel, NewProxyHostModel, ProxyHostModel,
        ProxyHostUpstreamModel, UpdateProxyHost, UpstreamModel,
    },
    repos::RepositoryError,
};

pub struct ProxyHostsRepository;

impl ProxyHostsRepository {
    pub async fn create(
        model: NewProxyHostModel,
        upstream_ids: Vec<i32>,
        database: &Database,
    ) -> Result<
        (
            ProxyHostModel,
            Vec<UpstreamModel>,
            Option<(AccessListModel, Vec<AccessListRuleModel>)>,
        ),
        RepositoryError,
    > {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::access_list_rules::dsl::{
                    access_list_id as rule_access_list_id, access_list_rules,
                };
                use crate::database::schema::access_lists::dsl::{access_lists, id as al_id};
                use crate::database::schema::proxy_host_upstreams::dsl::proxy_host_upstreams;
                use crate::database::schema::proxy_hosts::dsl::proxy_hosts;
                use crate::database::schema::upstreams::dsl::{id as upstream_id_col, upstreams};
                use diesel::prelude::*;

                conn.transaction(|conn| {
                    let host = diesel::insert_into(proxy_hosts)
                        .values(&model)
                        .get_result::<ProxyHostModel>(conn)?;

                    let links: Vec<ProxyHostUpstreamModel> = upstream_ids
                        .iter()
                        .map(|&uid| ProxyHostUpstreamModel {
                            proxy_host_id: host.id,
                            upstream_id: uid,
                        })
                        .collect();

                    diesel::insert_into(proxy_host_upstreams)
                        .values(&links)
                        .execute(conn)?;

                    let host_upstreams = upstreams
                        .filter(upstream_id_col.eq_any(&upstream_ids))
                        .load::<UpstreamModel>(conn)?;

                    let access_list = host
                        .access_list_id
                        .map(
                            |alid| -> diesel::QueryResult<(
                                AccessListModel,
                                Vec<AccessListRuleModel>,
                            )> {
                                let list = access_lists
                                    .filter(al_id.eq(alid))
                                    .first::<AccessListModel>(conn)?;

                                let rules = access_list_rules
                                    .filter(rule_access_list_id.eq(alid))
                                    .load::<AccessListRuleModel>(conn)?;

                                Ok((list, rules))
                            },
                        )
                        .transpose()?;

                    Ok((host, host_upstreams, access_list))
                })
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn update_full(
        host_id: i32,
        update: UpdateProxyHost,
        database: &Database,
    ) -> Result<
        (
            ProxyHostModel,
            Vec<UpstreamModel>,
            Option<(AccessListModel, Vec<AccessListRuleModel>)>,
        ),
        RepositoryError,
    > {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::access_list_rules::dsl::{
                    access_list_id as rule_access_list_id, access_list_rules,
                };
                use crate::database::schema::access_lists::dsl::{access_lists, id as al_id};
                use crate::database::schema::proxy_host_upstreams::dsl::{
                    proxy_host_id, proxy_host_upstreams,
                };
                use crate::database::schema::proxy_hosts::dsl::{
                    access_list_id, certificate_name, id, proxy_hosts,
                };
                use crate::database::schema::upstreams::dsl::upstreams;
                use diesel::prelude::*;

                conn.transaction(|conn| {
                    // Apply each field only if present in the request
                    let mut host = diesel::update(proxy_hosts.filter(id.eq(host_id)))
                        .set(&update.model)
                        .get_result::<ProxyHostModel>(conn)?;

                    if let Some(cert) = update.model.certificate_name {
                        host = diesel::update(proxy_hosts.filter(id.eq(host_id)))
                            .set(certificate_name.eq(cert))
                            .get_result::<ProxyHostModel>(conn)?;
                    }

                    if let Some(alid) = update.model.access_list_id {
                        host = diesel::update(proxy_hosts.filter(id.eq(host_id)))
                            .set(access_list_id.eq(alid))
                            .get_result::<ProxyHostModel>(conn)?;
                    }

                    if let Some(ref ids) = update.upstream_ids {
                        diesel::delete(proxy_host_upstreams.filter(proxy_host_id.eq(host_id)))
                            .execute(conn)?;

                        let links: Vec<ProxyHostUpstreamModel> = ids
                            .iter()
                            .map(|&uid| ProxyHostUpstreamModel {
                                proxy_host_id: host_id,
                                upstream_id: uid,
                            })
                            .collect();

                        diesel::insert_into(proxy_host_upstreams)
                            .values(&links)
                            .execute(conn)?;
                    }

                    let host_upstreams = proxy_host_upstreams
                        .filter(proxy_host_id.eq(host_id))
                        .inner_join(upstreams)
                        .select(UpstreamModel::as_select())
                        .load::<UpstreamModel>(conn)?;

                    let access_list = host
                        .access_list_id
                        .map(|alid| -> diesel::QueryResult<_> {
                            let list = access_lists
                                .filter(al_id.eq(alid))
                                .first::<AccessListModel>(conn)?;

                            let rules = access_list_rules
                                .filter(rule_access_list_id.eq(alid))
                                .load::<AccessListRuleModel>(conn)?;

                            Ok((list, rules))
                        })
                        .transpose()?;

                    Ok((host, host_upstreams, access_list))
                })
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
