use serde_json::Value as Json;
use toml::Value as Toml;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use log::info;

/// 把 toml 数据格式转换为 json 格式
/// 
/// ```rust
/// use crate::core::util::{init_log4rs, load_app_config, toml_to_json};
/// let app_config_toml:Toml = load_app_config("src/config/app.toml");
/// let app_config_json = toml_to_json(app_config_toml);
/// 
/// ```
pub fn toml_to_json(toml: Toml) -> Json {
    match toml {
        Toml::String(s) => Json::String(s),
        Toml::Integer(i) => Json::Number(i.into()),
        Toml::Float(f) => {
            let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
            Json::Number(n)
        }
        Toml::Boolean(b) => Json::Bool(b),
        Toml::Array(arr) => Json::Array(arr.into_iter().map(toml_to_json).collect()),
        Toml::Table(table) => {
            Json::Object(table.into_iter().map(|(k, v)| (k, toml_to_json(v))).collect())
        }
        Toml::Datetime(dt) => Json::String(dt.to_string()),
    }
}


/// 加载项目配置，包括路由配置，数据库配置，缓存配置，多语言配置等
///
/// 
/// ```rust
/// extern crate toml;
/// 
/// use toml::{Value as Toml};
/// 
/// let app_config:Toml = load_app_config("src/config/app.toml");
/// 
/// ```
pub fn load_app_config(config_file:&str) -> Toml {
    let path = Path::new(config_file);
    let mut file = File::open(&path).expect("Open app config fail !!! ");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("read file content fail");
    let config:Toml = toml::from_str(&file_content).unwrap();
    config
}


/// 初始化日志，通过读取log4rs.yaml配置文件来完成日志功能
/// 支持日志分割，支持全局logid，支持日志写入文件，支持actix-web日志中间件
/// 
/// 读取 app.toml logger 的配置，如果没有配置，默认显示 warn 级别以上的日志
/// [logger]
/// # trace, debug, info, warn, error
/// actix_web = "debug"
/// actix_redis = "info"
/// 
/// ```rust
/// use log::*;
/// 
/// `debug`, `info`, `warn`, `error`, or `trace`
/// debug!("this is a debug {}", "message");
/// error!("this is printed by default");
/// warn!("the answer was");
/// 
/// ```
pub fn init_log4rs(log_config: &Json) {
    let mut log_filter_level_vec = Vec::new();
    for (k,v) in  log_config.as_object().unwrap() {
        log_filter_level_vec.push(format!("{}={}",k,v));
    }
    if log_filter_level_vec.is_empty() {
        std::env::set_var("RUST_LOG", "actix_web=warn,actix_redis=warn");
    }else{
        std::env::set_var("RUST_LOG", log_filter_level_vec.join(","));
    }
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("log4rs initialized.");
}