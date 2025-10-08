use crate::{
    ctx::Ctx,
    model::{ModelManager, base::DbBmc},
};
use lib_auth::pwd::{self, ContentToHash};
use lib_utils::{b58::b58_encode, time::now_utc};
use rand::RngCore as _;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

mod error;

pub use error::{Error, Result};

// region:    --- User Types
#[derive(
    Clone, Debug, sqlx::Type, Deserialize, Serialize, strum_macros::Display,
)]
#[sqlx(type_name = "user_typ")]
pub enum UserTyp {
    Sys,
    User,
    UnVarifiedUser,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub typ: UserTyp,
}

/// Fields required for creating new user
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserForCreate {
    pub name: String,
    pub email: String,
    #[serde(rename = "password")]
    pub pwd_clear: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForLogin {
    pub user_id: String,
    pub name: String,
    pub typ: UserTyp,
    pub email: String,

    // -- pwd and token info
    /// encrypted, #_scheme_id_#....
    pub pwd: String,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForAuth {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub typ: UserTyp,

    // -- token info
    pub token_salt: Uuid,
}

// endregion: --- User Types

// region:    --- UserBmc

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    fn generate_user_id() -> String {
        let mut key = [0u8; 64]; // 512 bits = 64 bytes
        rand::rng().fill_bytes(&mut key);
        b58_encode(key)
            .chars()
            .take(10)
            .collect::<String>()
            .to_uppercase()
    }

    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        user_c: UserForCreate,
    ) -> Result<String> {
        let UserForCreate {
            name,
            pwd_clear,
            email,
        } = user_c;

        // Start the transaction
        let mm = mm.new_with_txn();

        mm.dbx().begin_txn().await?;

        let user_id = UserBmc::generate_user_id();
        let pwd_salt = pwd::generate_random_uuid_v4().await?;

        let pwd = pwd::hash_pwd(ContentToHash {
            content: pwd_clear.to_string(),
            salt: pwd_salt,
        })
        .await?;

        let now = now_utc();

        let sqlx_query = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO users (user_id, name, email, cid, mid, ctime, mtime) values ($1, $2, $3,
            (SELECT serial_id FROM users WHERE user_id = $4 LIMIT 1),
            (SELECT serial_id FROM users WHERE user_id = $4 LIMIT 1),
            $5, $5 ) returning serial_id",
        )
        .bind(user_id.as_str())
        .bind(name)
        .bind(email)
        .bind(ctx.user_id())
        .bind(now);

        // NOTE: For now, we will use the _txn for all create.
        //       We could have a with_txn as function argument if perf is an issue (it should not be)
        let (serial_id,) = mm.dbx().fetch_one(sqlx_query).await?;

        let sqlx_query = sqlx::query(
            "INSERT INTO auth (user_serial_id, pwd, pwd_salt, cid, mid, ctime, mtime) values ($1, $2, $3,
            (SELECT serial_id FROM users WHERE user_id = $4 LIMIT 1),
            (SELECT serial_id FROM users WHERE user_id = $4 LIMIT 1), $5, $5)",
        )
        .bind(serial_id)
        .bind(pwd)
        .bind(pwd_salt)
        .bind(ctx.user_id())
        .bind(now);

        mm.dbx().execute(sqlx_query).await?;

        // Commit the transaction
        mm.dbx().commit_txn().await?;

        Ok(user_id)
    }

    pub async fn get_by_user_id(
        _ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
    ) -> Result<UserForLogin> {
        let sqlx_query = sqlx::query_as::<_, UserForLogin>(
            "SELECT user_id, typ, name, email, pwd, pwd_salt, token_salt FROM users
                INNER JOIN auth ON serial_id = user_serial_id
                WHERE user_id = $1 LIMIT 1;",
        )
        .bind(user_id);

        let user = mm.dbx().fetch_optional(sqlx_query).await?.ok_or(
            Error::UserNotFound {
                user_id: user_id.into(),
            },
        )?;

        Ok(user)
    }

    pub async fn get_by_email(
        _ctx: &Ctx,
        mm: &ModelManager,
        email: &str,
    ) -> Result<UserForLogin> {
        let sqlx_query = sqlx::query_as::<_, UserForLogin>(
            "SELECT user_id, typ, name, email, pwd, pwd_salt, token_salt FROM users
                INNER JOIN auth ON serial_id = user_serial_id
                WHERE email = $1 LIMIT 1;",
        )
        .bind(email);

        let user = mm
            .dbx()
            .fetch_optional(sqlx_query)
            .await?
            .ok_or(Error::UserEmailNotFound)?;

        Ok(user)
    }

    pub async fn get_user_auth_by_email(
        _ctx: &Ctx,
        mm: &ModelManager,
        email: &str,
    ) -> Result<Option<UserForAuth>> {
        let sqlx_query = sqlx::query_as::<_, UserForAuth>(
            "SELECT user_id, typ, name, email, token_salt FROM users
                INNER JOIN auth ON serial_id = user_serial_id
                WHERE email = $1 LIMIT 1;",
        )
        .bind(email);

        let user_for_auth = mm.dbx().fetch_optional(sqlx_query).await?;

        Ok(user_for_auth)
    }

    pub async fn first_by_user_id(
        _ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
    ) -> Result<Option<UserForAuth>> {
        let sqlx_query = sqlx::query_as::<_, UserForAuth>(
            "SELECT user_id, typ, name, email, token_salt FROM users
                INNER JOIN auth ON serial_id = user_serial_id
                WHERE user_id = $1 LIMIT 1;",
        )
        .bind(user_id);

        let user_for_login = mm.dbx().fetch_optional(sqlx_query).await?;

        Ok(user_for_login)
    }

    pub async fn first_by_email(
        _ctx: &Ctx,
        mm: &ModelManager,
        email: &str,
    ) -> Result<Option<UserForAuth>> {
        let sqlx_query = sqlx::query_as::<_, UserForAuth>(
            "SELECT user_id, typ, name, email, token_salt FROM users
                INNER JOIN auth ON serial_id = user_serial_id
                WHERE email = $1 LIMIT 1;",
        )
        .bind(email);

        let user_for_login = mm.dbx().fetch_optional(sqlx_query).await?;

        Ok(user_for_login)
    }

    pub async fn list(
        _ctx: &Ctx,
        mm: &ModelManager,
        // filter: Option<Vec<UserFilter>>,
        // list_options: Option<ListOptions>,
    ) -> Result<Vec<User>> {
        let sqlx_query = sqlx::query_as::<_, User>(
            "SELECT user_id, name, email, typ FROM users",
        );

        let users = mm.dbx().fetch_all(sqlx_query).await?;

        Ok(users)
    }

    pub async fn update_pwd(
        ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
        pwd_clear: &str,
    ) -> Result<()> {
        // -- Prep password
        let user: UserForLogin = Self::get_by_user_id(ctx, mm, user_id).await?;

        let pwd = pwd::hash_pwd(ContentToHash {
            content: pwd_clear.to_string(),
            salt: user.pwd_salt,
        })
        .await?;

        let now = now_utc();

        let sqlx_query = sqlx::query(
            "UPDATE auth SET
              pwd = $2,
              mid = (SELECT serial_id FROM users WHERE user_id = $3 LIMIT 1),
              mtime = $4
            WHERE user_serial_id = (SELECT serial_id FROM users
              WHERE user_id = $1);",
        )
        .bind(user_id)
        .bind(pwd)
        .bind(ctx.user_id())
        .bind(now);

        let _count = mm.dbx().execute(sqlx_query).await?;

        Ok(())
    }

    /// TODO: For User, deletion will require a soft-delete approach:
    ///       - Set `deleted: true`.
    ///       - Change `username` to "DELETED-_user_id_".
    ///       - Clear any other UUIDs or PII (Personally Identifiable Information).
    ///       - The automatically set `mid`/`mtime` will record who performed the deletion.
    ///       - It's likely necessary to record this action in a `um_change_log` (a user management change audit table).
    ///       - Remove or clean up any user-specific assets (messages, etc.).
    pub async fn delete(
        _ctx: &Ctx,
        mm: &ModelManager,
        user_id: &str,
    ) -> Result<()> {
        let sqlx_query = sqlx::query(
            "DELETE FROM users 
                WHERE user_id = $1;",
        )
        .bind(user_id);

        let _count = mm.dbx().execute(sqlx_query).await?;

        Ok(())
    }
}

// endregion: --- UserBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
    pub type Result<T> = std::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>; // For tests.

    use super::*;
    use crate::_dev_utils;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok-user-01";
        let fx_email = "test_create_ok-user-user01@test.com";
        let fx_pwd_clear = "test_create_ok pwd 01";

        // -- Exec
        let user_id = UserBmc::create(
            &ctx,
            &mm,
            UserForCreate {
                name: fx_name.to_string(),
                email: fx_email.to_string(),
                pwd_clear: fx_pwd_clear.to_string(),
            },
        )
        .await?;

        // -- Check
        let user: UserForLogin =
            UserBmc::get_by_user_id(&ctx, &mm, &user_id).await?;
        assert_eq!(user.name, fx_name);

        // -- Clean
        UserBmc::delete(&ctx, &mm, &user_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_user_id = "demo1";

        // -- Exec
        let user = UserBmc::first_by_user_id(&ctx, &mm, fx_user_id)
            .await?
            .ok_or("Should have user 'demo1'")?;

        // -- Check
        assert_eq!(user.user_id, fx_user_id);

        Ok(())
    }
}

// endregion: --- Tests
