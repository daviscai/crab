use actix_web::{
    middleware,
    App,
    HttpServer
};
// use actix_service::Service;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use r2d2_redis::{r2d2, RedisConnectionManager};
// use r2d2_redis_cluster::{RedisClusterConnectionManager};

use actix_cors::Cors;

// use actix_session::CookieSession;
use serde_json::Value as Json;
use toml::Value as TomlValue;
use uuid::Uuid;

use crate::core::db::DBContext;
use crate::core::i18n::{I18N};
use crate::core::redis::{RedisContext}; //RedisClusterContext
use crate::core::util::{init_log4rs, load_app_config, toml_to_json};
// use crate::AppToolkit;

// use crate::controllers::app::home;
use crate::config::router;

use diesel::r2d2::{ConnectionManager};
use diesel::*; 

// use std::ops::{Deref, DerefMut};

// mysql 单机模式连接池
type MysqlPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// 单机模式连接池
type RedisPool = r2d2::Pool<RedisConnectionManager>;

// 集群模式连接池
// type RedisClusterPool = r2d2::Pool<RedisClusterConnectionManager>;

// #[derive(Serialize, Deserialize, Debug)]
// #[derive(Debug)]
pub struct AppToolkit {
    pub config: Json,
    pub uuid: String,
    pub redis_pool: RedisPool,
    pub db_pool: MysqlPool,
    pub locale: I18N
}

// impl Deref for AppToolkit {
//     type Target = I18N;
//     fn deref(&self) -> &Self::Target {
//         &self
//     }
// }

// impl DerefMut for AppToolkit {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self
//     }
// }


pub struct AppServer {}

impl AppServer {
    pub async fn start_http() -> std::io::Result<()> {
        // 读取应用配置
        let app_config_toml: TomlValue = load_app_config("src/config/app.toml");
        let app_config_json: Json = toml_to_json(app_config_toml);
    
        // 初始化日志
        let config = app_config_json.clone();
        init_log4rs(&app_config_json["logger"]);

        // 监听的ip 端口
        let server_host = config["server"]["http_host"].as_str().unwrap();

        // mysql pool
        let db_pool = DBContext::new_mysql_pool(app_config_json.clone());

        // redis pool
        let redis_pool = RedisContext::new_pool(app_config_json.clone());

        // redis cluster pool
        // let redis_pool = RedisContext::new_cluster_pool(app_config_json.clone());

        // uuid 
        let uuid = Uuid::new_v4().to_string();

         // i18n
         let i18n = I18N::configure(app_config_json.clone());

        HttpServer::new(move || {
            // 设置全局唯一的 logid ， log4rs 日志库会读取key=log_id的值写入日志文件
            log_mdc::insert("log_id", &uuid);
            let cors =  get_cors(app_config_json.clone());

            let apptoolkit = AppToolkit {
                config : app_config_json.clone(),
                uuid : uuid.clone(), 
                redis_pool : redis_pool.clone(), 
                db_pool : db_pool.clone(),
                locale: i18n.clone()
            };

            App::new()
            .wrap(cors.finish())
            .wrap(i18n.clone())
            .wrap(middleware::Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %D"#))
            /* .wrap(
                RedisSession::new("127.0.0.1:6379", &[0; 32])
                // allow the cookie to be accessed from javascript
                //.cookie_http_only(false) <- master分支才有这个方法
                // allow the cookie only from the current domain
                .cookie_same_site(cookie::SameSite::Strict)
            )*/
            // .wrap(
            //     CookieSession::signed(&[0; 32])
            //         .domain("127.0.0.1") // 不能带端口
            //         .name("sessionid")
            //         .path("/")
            //         .secure(false) // https 协议下才能设置为true
            //         .http_only(false) // 设置为true，前端js将不能通过 document.cookie 读取
            //         .max_age(3600), // second
            // ) // 如果使用redis session ，该中间件可以去掉
            .data(apptoolkit)
            .configure(router::routes)
           
        })
        .bind(&server_host)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", server_host))
        .run().await
    }


    pub async fn start_https() -> std::io::Result<()> {

        // 读取应用配置
        let app_config_toml: TomlValue = load_app_config("src/config/app.toml");
        let app_config_json: Json = toml_to_json(app_config_toml);

        // 设置ssl公钥和证书
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file("src/config/rsa_private_key.pem", SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file("src/config/cert.pem").unwrap();
     
        // 初始化日志
        let config = app_config_json.clone();
        init_log4rs(&app_config_json["logger"]);

        // 监听的ip 端口
        let http_server_host = config["server"]["http_host"].as_str().unwrap();
        let https_server_host = config["server"]["https_host"].as_str().unwrap();

        // mysql pool
        let db_pool = DBContext::new_mysql_pool(app_config_json.clone());

        // redis pool
        let redis_pool = RedisContext::new_pool(app_config_json.clone());

        // redis cluster pool
        // let redis_pool = RedisContext::new_cluster_pool(app_config_json.clone());

        // uuid 
        let uuid = Uuid::new_v4().to_string();
       
        HttpServer::new( move || {
            // 设置全局唯一的 logid ， log4rs 日志库会读取key=log_id的值写入日志文件
            log_mdc::insert("log_id", &uuid);
            let cors =  get_cors(app_config_json.clone());

            // i18n
            let i18n = I18N::configure(app_config_json.clone());

            let apptoolkit = AppToolkit {
                config : app_config_json.clone(),
                uuid : uuid.clone(), 
                redis_pool : redis_pool.clone(), 
                db_pool : db_pool.clone(),
                locale: i18n.clone()
            };

            App::new()
                .wrap(cors.finish())
                .wrap(i18n)
                // .wrap_fn(|req, srv| {
                    
                //     //i18n.set_locale_lang(req.headers().get("accept-language").unwrap().to_str().unwrap());

                //     let fut = srv.call(req);
                //     async {
                //         let mut res = fut.await?;
                //         Ok(res)
                //     }
                // })
                .wrap(middleware::Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %D"#))  // enable logger
                .data(apptoolkit)
                .configure(router::routes)
        })
        .bind(&http_server_host)  // 绑定 http 端口
        .unwrap_or_else(|_| panic!("Could not bind http server to address {}", http_server_host))
        .bind_openssl(&https_server_host, builder) // 绑定 https 端口
        .unwrap_or_else(|_| panic!("Could not bind https server to address {}", https_server_host))
        .run().await
    }

}

fn get_cors(config: Json) -> Cors{
    let check_cross_origin = config["cors"]["check_cross_origin"].as_bool();
    let cors_origin = config["cors"]["origin"].as_str().unwrap();
    let mut methods = Vec::new();
    for item in config["cors"]["methods"].as_array().unwrap() {
        methods.push(item.as_str().unwrap());
    }
    let max_age = config["cors"]["max_age"].as_u64().unwrap() as usize;

    let cors = match check_cross_origin {
        Some(true) => { 
            Cors::new().allowed_origin(cors_origin).allowed_methods(methods).max_age(max_age).supports_credentials() 
        },
        Some(false) => Cors::new(),
        None => Cors::new().supports_credentials() 
    };
    cors 
}