use crate::database::{
    Database,
    models::{AccessListModel, AccessListRuleModel, ProxyHostModel, UpstreamModel},
    repos::RepositoryError,
};

pub struct ProxyHostUpstreamsRepository;

impl ProxyHostUpstreamsRepository {
    pub async fn get_all_with_upstreams(
        database: &Database,
        page: i64,
        per_page: i64,
    ) -> Result<
        Vec<(
            ProxyHostModel,
            Vec<UpstreamModel>,
            Option<(AccessListModel, Vec<AccessListRuleModel>)>,
        )>,
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
                use crate::database::schema::proxy_hosts::dsl::{id, proxy_hosts};
                use crate::database::schema::upstreams::dsl::upstreams;
                use diesel::prelude::*;

                let hosts = proxy_hosts
                    .order(id.asc())
                    .limit(per_page)
                    .offset((page - 1) * per_page)
                    .load::<ProxyHostModel>(conn)?;

                let host_ids: Vec<i32> = hosts.iter().map(|h| h.id).collect();

                let upstream_pairs = proxy_host_upstreams
                    .filter(proxy_host_id.eq_any(&host_ids))
                    .inner_join(upstreams)
                    .select((proxy_host_id, UpstreamModel::as_select()))
                    .load::<(i32, UpstreamModel)>(conn)?;

                let access_list_ids: Vec<i32> =
                    hosts.iter().filter_map(|h| h.access_list_id).collect();

                let fetched_access_lists = access_lists
                    .filter(al_id.eq_any(&access_list_ids))
                    .load::<AccessListModel>(conn)?;

                let fetched_rules = access_list_rules
                    .filter(rule_access_list_id.eq_any(&access_list_ids))
                    .load::<AccessListRuleModel>(conn)?;

                let result = hosts
                    .into_iter()
                    .map(|host| {
                        let host_upstreams = upstream_pairs
                            .iter()
                            .filter(|(pid, _)| *pid == host.id)
                            .map(|(_, u)| u.clone())
                            .collect::<Vec<_>>();

                        let access_list = host.access_list_id.and_then(|alid| {
                            let list = fetched_access_lists
                                .iter()
                                .find(|al| al.id == alid)
                                .cloned()?;
                            let rules = fetched_rules
                                .iter()
                                .filter(|r| r.access_list_id == alid)
                                .cloned()
                                .collect::<Vec<_>>();
                            Some((list, rules))
                        });

                        (host, host_upstreams, access_list)
                    })
                    .collect();

                Ok(result)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }
}
