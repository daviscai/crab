use serde_json::Value as Json;
//use log::*;
//use redis::{RedisResult};
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use std::time::{Duration};

use r2d2_redis_cluster::{RedisClusterConnectionManager};

type RedisPool = r2d2::Pool<RedisConnectionManager>;
type RedisClusterPool = r2d2::Pool<RedisClusterConnectionManager>;


#[derive(Clone)]
pub struct RedisContext {
    pub client: redis::Client
}

impl RedisContext {

    pub fn new_pool(config: Json) -> RedisPool {
        let manager = RedisConnectionManager::new(config["redis"]["host"][0].as_str().unwrap()).unwrap();
        let pool_max_size = config["redis"]["pool_max_size"].as_u64().unwrap() as u32;
        let connection_timeout = config["redis"]["connection_timeout"].as_u64();
        let timeout = match connection_timeout {
            Some(t) => t,
            None => 250
        };
        r2d2::Pool::builder()
            .max_size(pool_max_size)
            .connection_timeout(Duration::from_millis(timeout))
            .build(manager)
            .unwrap()
    }

    pub fn new_cluster_pool(config: Json) -> RedisClusterPool {
        let mut nodes = Vec::new();
        for node in config["redis"]["host"].as_array().unwrap() {
            nodes.push(node.as_str().unwrap());
        }

        let pool_max_size = config["redis"]["pool_max_size"].as_u64().unwrap() as u32;
        let manager = RedisClusterConnectionManager::new(nodes).unwrap();
        let connection_timeout = config["redis"]["connection_timeout"].as_u64();
        let timeout = match connection_timeout {
            Some(t) => t,
            None => 250
        }; 
        r2d2::Pool::builder()
            .max_size(pool_max_size)
            .connection_timeout(Duration::from_millis(timeout))
            .build(manager)
            .unwrap()
    }

}