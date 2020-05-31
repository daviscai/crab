// extern crate r2d2;

use diesel::r2d2::{ConnectionManager};
use diesel::*; 
use std::time::{Duration};
use serde_json::Value as Json;


type MysqlPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct DBContext {}

impl DBContext {

    pub fn new_mysql(config: Json) -> MysqlConnection {
        let url = config["mysql"]["url"].as_str().unwrap();
        MysqlConnection::establish(&url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", url))
    }
    
    pub fn new_mysql_pool(config: Json) -> MysqlPool {
        let url = config["mysql"]["url"].as_str().unwrap();
        let pool_max_size = config["mysql"]["pool_max_size"].as_u64().unwrap() as u32;
        let manager = ConnectionManager::<MysqlConnection>::new(url);
        r2d2::Pool::builder()
        .max_size(pool_max_size)
        .connection_timeout(Duration::from_millis(250))
        .build(manager)
        .unwrap()
    }

}