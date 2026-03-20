use crate::database::{
    Database,
    models::{NewUpstreamModel, UpdateUpstreamModel, UpstreamModel},
    repos::RepositoryError,
};

pub struct UpstreamsRepository;

impl UpstreamsRepository {
    pub async fn get_all(
        database: &Database,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<UpstreamModel>, i64), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::upstreams::dsl::{id, upstreams};
                use diesel::prelude::*;

                let items = upstreams
                    .order(id.asc())
                    .limit(per_page)
                    .offset((page - 1) * per_page)
                    .load::<UpstreamModel>(conn)?;

                let total = upstreams.count().get_result::<i64>(conn)?;

                Ok((items, total))
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn create(
        model: NewUpstreamModel,
        database: &Database,
    ) -> Result<UpstreamModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::upstreams::dsl::upstreams;
                use diesel::prelude::*;
                diesel::insert_into(upstreams)
                    .values(&model)
                    .get_result::<UpstreamModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn update(
        upstream_id: i32,
        model: UpdateUpstreamModel,
        database: &Database,
    ) -> Result<UpstreamModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::upstreams::dsl::{id, upstreams};
                use diesel::prelude::*;
                diesel::update(upstreams.filter(id.eq(upstream_id)))
                    .set(&model)
                    .get_result::<UpstreamModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn delete(upstream_id: i32, database: &Database) -> Result<(), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::upstreams::dsl::{id, upstreams};
                use diesel::prelude::*;
                diesel::delete(upstreams.filter(id.eq(upstream_id)))
                    .execute(conn)
                    .map(|_| ())
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }
}
