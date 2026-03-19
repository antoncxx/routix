use crate::database::{
    Database,
    models::{
        AccessListModel, AccessListRuleModel, NewAccessListModel, NewAccessListRuleModel,
        UpdateAccessListModel,
    },
    repos::RepositoryError,
};

pub struct AccessListsRepository;

impl AccessListsRepository {
    pub async fn create(
        model: NewAccessListModel,
        rules: Vec<NewAccessListRuleModel>,
        database: &Database,
    ) -> Result<(AccessListModel, Vec<AccessListRuleModel>), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::access_list_rules::dsl::access_list_rules;
                use crate::database::schema::access_lists::dsl::access_lists;
                use diesel::prelude::*;

                conn.transaction(|conn| {
                    let access_list = diesel::insert_into(access_lists)
                        .values(&model)
                        .get_result::<AccessListModel>(conn)?;

                    let rules: Vec<NewAccessListRuleModel> = rules
                        .into_iter()
                        .map(|r| NewAccessListRuleModel {
                            access_list_id: access_list.id,
                            ..r
                        })
                        .collect();

                    let rules = diesel::insert_into(access_list_rules)
                        .values(&rules)
                        .get_results::<AccessListRuleModel>(conn)?;

                    Ok((access_list, rules))
                })
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn fetch(
        list_id: i32,
        database: &Database,
    ) -> Result<(AccessListModel, Vec<AccessListRuleModel>), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::access_list_rules::dsl::{
                    access_list_id, access_list_rules, sort_order,
                };
                use crate::database::schema::access_lists::dsl::{access_lists, id};
                use diesel::prelude::*;

                let access_list = access_lists
                    .filter(id.eq(list_id))
                    .first::<AccessListModel>(conn)?;

                let rules = access_list_rules
                    .filter(access_list_id.eq(list_id))
                    .order(sort_order.asc())
                    .load::<AccessListRuleModel>(conn)?;

                Ok((access_list, rules))
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
                use crate::database::schema::access_lists::dsl::access_lists;
                use diesel::prelude::*;
                access_lists.count().get_result::<i64>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn fetch_all(
        database: &Database,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<(AccessListModel, Vec<AccessListRuleModel>)>, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::access_list_rules::dsl::sort_order;
                use crate::database::schema::access_lists::dsl::access_lists;
                use diesel::prelude::*;

                let lists = access_lists
                    .limit(per_page)
                    .offset((page - 1) * per_page)
                    .load::<AccessListModel>(conn)?;

                let rules = AccessListRuleModel::belonging_to(&lists)
                    .order(sort_order.asc())
                    .load::<AccessListRuleModel>(conn)?;

                let result = rules
                    .grouped_by(&lists)
                    .into_iter()
                    .zip(lists)
                    .map(|(rules, access_list)| (access_list, rules))
                    .collect();

                Ok(result)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn set_rules(
        list_id: i32,
        rules: Vec<NewAccessListRuleModel>,
        database: &Database,
    ) -> Result<(AccessListModel, Vec<AccessListRuleModel>), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::access_list_rules::dsl::{
                    access_list_id, access_list_rules,
                };
                use crate::database::schema::access_lists::dsl::{access_lists, id};
                use diesel::prelude::*;

                conn.transaction(|conn| {
                    let access_list = access_lists
                        .filter(id.eq(list_id))
                        .first::<AccessListModel>(conn)?;

                    diesel::delete(access_list_rules.filter(access_list_id.eq(list_id)))
                        .execute(conn)?;

                    let rules = diesel::insert_into(access_list_rules)
                        .values(&rules)
                        .get_results::<AccessListRuleModel>(conn)?;

                    Ok((access_list, rules))
                })
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn update(
        list_id: i32,
        model: UpdateAccessListModel,
        database: &Database,
    ) -> Result<(AccessListModel, Vec<AccessListRuleModel>), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::access_list_rules::dsl::{
                    access_list_id, access_list_rules, sort_order,
                };
                use crate::database::schema::access_lists::dsl::{access_lists, id};
                use diesel::prelude::*;

                conn.transaction(|conn| {
                    let access_list = diesel::update(access_lists.filter(id.eq(list_id)))
                        .set(&model)
                        .get_result::<AccessListModel>(conn)?;

                    let rules = access_list_rules
                        .filter(access_list_id.eq(list_id))
                        .order(sort_order.asc())
                        .load::<AccessListRuleModel>(conn)?;

                    Ok((access_list, rules))
                })
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn delete(list_id: i32, database: &Database) -> Result<(), RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::access_lists::dsl::{access_lists, id};
                use diesel::prelude::*;
                diesel::delete(access_lists.filter(id.eq(list_id)))
                    .execute(conn)
                    .map(|_| ())
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }
}
