use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::*;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_pool(database_url: &String) -> MysqlPool {
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::builder().max_size(100).build(manager).unwrap()
}
