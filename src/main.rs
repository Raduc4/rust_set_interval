#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

mod api;
mod config;
mod constants;
mod error;
mod models;
mod schema;
mod services;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use models::names::NamesRepo;
use services::names_service::create_name;
use std::default::Default;

use std::{env, io};
use tokio_js_set_interval::{set_interval, set_interval_async, set_timeout_async};

use crate::models::names::NewNameDTO;

async fn async_fn_prints_the_name(name: &str) {
    println!("{name}");
}

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_host = env::var(constants::APP_HOST).expect("APP_HOST not found.");
    let app_port = env::var(constants::APP_PORT).expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var(constants::DATABASE_URL).expect("DATABASE_URL not found.");
    let pool = config::db::migrate_and_config_db(&db_url);
    // This works
    set_interval_async!(
        {
            async move {
                async_fn_prints_the_name("Name").await;
            }
        },
        42
    );

    set_interval_async!(
        move || {
            let names_repo = NamesRepo::new(pool.clone());

            async move { create_name(names_repo, NewNameDTO::new()) }
        },
        42
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %s %{User-Agent}i"))
        // .configure(config::app::config_services)
    })
    .bind(app_url)?
    .run()
    .await
}
