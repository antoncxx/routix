use crate::database::{
    Database,
    models::{NewProxyHostModel, ProxyHostModel},
    repos::RepositoryError,
};

pub struct ProxyHostsRepository;

impl ProxyHostsRepository {
    pub async fn get_all(database: &Database) -> Result<Vec<ProxyHostModel>, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::proxy_hosts::dsl::proxy_hosts;
                use diesel::prelude::*;
                proxy_hosts.load::<ProxyHostModel>(conn)
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
}
