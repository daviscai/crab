use actix_web::{
    web, 
    Responder, 
    //HttpRequest
    //Error, 
    //HttpResponse
};
// use crate::AppToolkit;
use log::*;
// use actix_session::{Session};
// use actix_http::cookie::Cookie;
use crate::core::app::{AppToolkit};

// use actix::prelude::*;
// use actix_redis::{Command, RedisActor, Error as Redis_Error };

// 非集群方式用这个
use redis::Commands;

// 集群方式用这个
// use r2d2_redis_cluster::{Commands as ClusterCommands};

// use diesel::prelude::*;
use crate::models::post::*;
// use crate::models::schema::posts::dsl::*;
// use crate::models::schema::posts::dsl::*;

use serde_json::json;
use serde::{Serialize, Deserialize};


pub async fn index(data: web::Data<AppToolkit>) -> impl Responder {
    
    test_mysql(&data);

    format!("Hello rust")

}

pub async fn test(data: web::Data<AppToolkit>, info: web::Path<(String, u32)>)  -> impl Responder {

    test_redis(&data);

    test_mysql(&data);

    test_json();

    format!("{}! id:{}", info.1, info.0)
}


fn test_redis(data: &web::Data<AppToolkit>) {
    // 读写 redis 
    let mut redis_con = data.redis_pool.get().unwrap();

    // 写 redis 
    let _: () = redis_con.set("test", "test_data").unwrap();

    // 读 redis
    let val: String = redis_con.get("test").unwrap();

    // 删除 redis

    debug!("redis data is: {}", val );
}


fn test_mysql(data: &web::Data<AppToolkit>) {
    // 获取mysql连接
    let connection = &data.db_pool.get().expect("couldn't get db connection from pool");
    
    // 写数据
    Post::insert(connection).expect("insert posts fail");

    // 查询数据
    let result = Post::find_by_id(connection, 1).expect("find posts fail");
    debug!("post title is: {}", result.title );

    // 删除数据

    
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
    weight: Option<u8>,
}

fn test_json() {
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    debug!("first phone number: {}", john["phones"][0]);
    debug!("{}", john.to_string());

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // 解析字符串到Person对象。
    let p: Person = serde_json::from_str(data).unwrap();
    debug!("Please call {} at the number {}", p.name, p.phones[0]);
    
    // Person对象转为JSON字符串.
    let serialized = serde_json::to_string(&p).unwrap();
    debug!("serialized = {}", serialized);

}

// // 基于 session 会话的数据存储和读取
// pub async fn test_session(session: Session)  -> Result<HttpResponse, Error> {
//     // access session data
//     if let Some(count) = session.get::<i32>("counter")? {
//         session.set("counter", count + 1)?;
//     } else {
//         session.set("counter", 1)?;
//     }

//     Ok(HttpResponse::Ok().body(format!(
//         "Count is {:?}!",
//         session.get::<i32>("counter")?.unwrap()
//     )))
// }

// pub async fn get_cookie(req: HttpRequest) -> impl Responder {
//     // 读取cookie值
//     let a = req.headers().get("cookie").unwrap();
//     let c = Cookie::parse(a.to_str().unwrap()).unwrap();
//     format!("cookie {} = {}", c.name(), c.value())
// }

// // 设置和访问cookie
// pub async fn set_cookie() -> impl Responder {
//     let cookie = Cookie::new("userid", "aaaaa");
//     format!("set cookie")
// }