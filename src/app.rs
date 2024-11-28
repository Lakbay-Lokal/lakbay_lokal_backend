use std::path::Path;

use async_trait::async_trait;
use axum::{middleware, Router};
use loco_rs::{
    app::{AppContext, Hooks},
    bgworker::Queue,
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    // db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::controllers;
use crate::middleware::auth::middleware_authentication;

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::route::routes())
            .add_route(controllers::station::routes())
            .add_route(controllers::ticketing::routes())
            .add_route(controllers::user::tracking::routes())
            .add_route(controllers::user::routes())
            .add_route(controllers::auth::routes())
            .add_route(controllers::bus::routes())
            .add_route(controllers::bus::tracking::routes())
            .add_route(controllers::bus::tracking_all::routes())
            .add_route(controllers::route::routes())
            .add_route(controllers::station::routes())
            .prefix("api")
    }

    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        // queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    fn register_tasks(_tasks: &mut Tasks) {
        // tasks.register(tasks::seed::SeedData);
    }

    async fn truncate(_db: &DatabaseConnection) -> Result<()> {
        // truncate_table(db, users::Entity).await?;
        Ok(())
    }

    async fn seed(_db: &DatabaseConnection, _base: &Path) -> Result<()> {
        // db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        Ok(())
    }

    async fn after_routes(router: Router, ctx: &AppContext) -> Result<Router> {
        let jwt_config = ctx
            .config
            .get_jwt_config()
            .map_err(|e| {
                tracing::error!("Failed to get jwt config: {:?}", e);
                e
            })?
            .clone();
        Ok(router.layer(middleware::from_fn_with_state(
            jwt_config,
            middleware_authentication,
        )))
    }
}
