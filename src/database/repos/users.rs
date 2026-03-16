use crate::database::{
    Database,
    models::{NewUserModel, UpdateUserModel, UserModel},
    repos::RepositoryError,
};

pub struct UsersRepository;

impl UsersRepository {
    pub async fn find_by_username(
        username: &str,
        database: &Database,
    ) -> Result<Option<UserModel>, RepositoryError> {
        let uname = username.to_owned();

        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::users::dsl::{username, users};
                use diesel::prelude::*;
                users
                    .filter(username.eq(&uname))
                    .first::<UserModel>(conn)
                    .optional()
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn create(
        model: NewUserModel,
        database: &Database,
    ) -> Result<UserModel, RepositoryError> {
        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::users::dsl::users;
                use diesel::prelude::*;
                diesel::insert_into(users)
                    .values(&model)
                    .get_result::<UserModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }

    pub async fn update(
        username: &str,
        model: UpdateUserModel,
        database: &Database,
    ) -> Result<UserModel, RepositoryError> {
        let uname = username.to_owned();

        let connection = database
            .connection()
            .await
            .map_err(RepositoryError::Connection)?;

        connection
            .interact(move |conn| {
                use crate::database::schema::users::dsl::{username, users};
                use diesel::prelude::*;
                diesel::update(users.filter(username.eq(&uname)))
                    .set(&model)
                    .get_result::<UserModel>(conn)
            })
            .await
            .map_err(RepositoryError::Interact)?
            .map_err(RepositoryError::Query)
    }
}
