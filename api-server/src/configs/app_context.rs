use std::error::Error;
use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use crate::configs::auth_config::AuthConfig;
use crate::configs::db_config::DatabaseConfig;
use crate::configs::redis_config::RedisConfig;
use crate::configs::system_config::SystemConfig;
use crate::log_info;

use redis::Client as RedisClient;
use tokio::sync::OnceCell;

/// Application context containing runtime configuration and core connections.
#[derive(Clone)]
pub struct AppContext {
    pub system: SystemConfig,
    pub db_pool: Pool<Postgres>,
    pub redis_config: RedisConfig,
    pub redis_client: Arc<RedisClient>,
    pub auth: AuthConfig,
}

impl AppContext {
    /// Initialize the application context from environment variables and
    /// establish the database connection pool.
    pub async fn initialize() -> Result<Self, Box<dyn Error>> {
        let system = SystemConfig::load_from_env()?;
        let database = DatabaseConfig::load_from_env()?;
        let redis_cfg = RedisConfig::load_from_env()?;
        let auth = AuthConfig::load_from_env()?;

        let db_pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database.database_url)
            .await?;

        log_info!("Connected to Postgres");

        // Initialize Redis client and verify connectivity once
        let redis_client = Arc::new(RedisClient::open(redis_cfg.redis_url.clone())?);
        {
            let mut conn = redis_client.get_multiplexed_tokio_connection().await?;
            let _: String = redis::cmd("PING").query_async(&mut conn).await?;
        }
        log_info!("Connected to Redis");

        Ok(Self {
            system,
            db_pool,
            redis_config: redis_cfg,
            redis_client,
            auth,
        })
    }

    /// Global singleton accessor. Ensures the context is initialized only once.
    pub async fn get() -> Result<&'static Arc<AppContext>, Box<dyn Error>> {
        static APP_CTX: OnceCell<Arc<AppContext>> = OnceCell::const_new();
        let ctx = APP_CTX
            .get_or_try_init(|| async {
                let ctx = AppContext::initialize().await?;
                Ok::<Arc<AppContext>, Box<dyn Error>>(Arc::new(ctx))
            })
            .await?;
        Ok(ctx)
    }
}
