#[allow(unused_imports)]
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
    sql_query,
};

embed_migrations!();

pub type Connection = PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn migrate_and_config_db(url: &str) -> Pool {
    info!("Configuring Database...");
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    embedded_migrations::run(&pool.get().expect("Failed to migrate.")).unwrap();
    pool
}
