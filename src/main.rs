// Rust crab 1.0 ， 目标：基础功能完善，可用于常见的云端API接口业务，使用门槛低，安全，高性能，低时延(<50ms)
// 1、支持 redis 集群和连接池，通过配置完成开箱即用 [done]
// 2、支持 mysql 连接池，开箱即用                [done]
// 3、支持日志功能                              [done]
// 4、支持 https                               [done]
// 5、友好的路由配置                            [done]
// 6、基于mvc模式                              [done]
// 7、支持html/js/css等前端页面渲染              [done]
// 8、Yew                                     [doing]
// 9、性能优异，评测数据，测试：写日志、读写数据库各1次、读写缓存各1次、路由解析、json编解码 [done]
//    测试结果：1、并发100持续10分钟情况下，258586次请求100%成功，平均时延230ms；2、并发10持续10分钟，231866次请求100%成功，平均时延30ms
//    测试环境：MacBook Pro ，8 GB内存，Intel Core i5 处理器，macOS: 10.14.6 
// 10、单元测试覆盖率100%，完善的文档              [todo]
//
// Rust wasm             Rust app server(mvc-rs)
// in browser <- REST -> |-------------------------------------|
//                       | HTTP Server -- actix-web            |
//  |                    |     |                               |
// Yew                   | Diesel (ORM) -> Mysql / PostgreSQL  |
//                       |-------------------------------------|
// 

// Rust crab 2.0， 目标：覆盖更多应用场景，支持国际化/本地化，支持IM即时通讯，支持图像和视频处理，满足更多企业级应用需求
// 1、支持 PostgreSQL 连接池，开箱即用
// 2、支持 i18n 本地化
// 3、支持 websocket 长连接
// 4、支持 消息队列，开箱即用
// 5、支持 前端模板语言
// 6、支持 auth2.0 ，开箱即用
// 7、强大的工具类，类似node.js的 underscore 库
// 8、支持图像和视频处理
// 9、支持 第三方应用接入，如 第三方账号登录和社交分享、支付等通用基础服务


// Rust crab 3.0 ，目标：支持千万级流量的高效运维，完善容灾，备份，限流，监控，预警方案，为企业级应用保驾护航 
// 1、支持docker容器


extern crate crab;
use crate::crab::core::app::{AppServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // // http server
    AppServer::start_http().await

    // https server
    // AppServer::start_https().await
}

/*
Transactions:		        2458 hits
Availability:		       69.77 %
Elapsed time:		       37.20 secs
Data transferred:	        0.03 MB
Response time:		        0.64 secs
Transaction rate:	       66.08 trans/sec
Throughput:		        0.00 MB/sec
Concurrency:		       42.25
Successful transactions:        2458
Failed transactions:	        1065
Longest transaction:	        2.12
Shortest transaction:	        0.01

Transactions: 30000 hits    //完成30000次处理  
Availability: 100.00 %      //100.00 % 成功率  
Elapsed time: 68.59 secs    //总共使用时间  
Data transferred: 817.76 MB //共数据传输 817.76 MB  
Response time: 0.04 secs    //响应时间，显示网络连接的速度  
Transaction rate: 437.38 trans/sec //平均每秒完成 437.38 次处理  
Throughput: 11.92 MB/sec    //平均每秒传送数据  
Concurrency: 17.53          //实际最高并发连接数  
Successful transactions: 30000 //成功处理次数  
Failed transactions: 0      //失败处理次数  
Longest transaction: 3.12   //每次传输所花最长时间  
Shortest transaction: 0.00   //每次传输所花最短时间 

1、release 环境，无业务逻辑，纯粹框架自身耗时
siege -c 200 -t 10 http://localhost:8080/ 
Transactions:		       99288 hits
Availability:		       98.87 %
Elapsed time:		      223.90 secs
Data transferred:	        0.95 MB
Response time:		        0.18 secs
Transaction rate:	      443.45 trans/sec
Throughput:		        0.00 MB/sec
Concurrency:		       79.44
Successful transactions:       99288
Failed transactions:	        1132
Longest transaction:	       19.98
Shortest transaction:	        0.00

siege -c 10 -t 10 http://localhost:8080/aaa/1223/test.html
Transactions:		      231866 hits
Availability:		      100.00 %
Elapsed time:		      599.11 secs
Data transferred:	        2.65 MB
Response time:		        0.03 secs
Transaction rate:	      387.02 trans/sec
Throughput:		        0.00 MB/sec
Concurrency:		        9.97
Successful transactions:      231866
Failed transactions:	           0
Longest transaction:	       20.05
Shortest transaction:	        0.00

siege -c 500 -t 10 http://localhost:8080/aaa/1223/test.html
Transactions:		      160988 hits
Availability:		       99.37 %
Elapsed time:		      494.65 secs
Data transferred:	        1.84 MB
Response time:		        0.78 secs
Transaction rate:	      325.46 trans/sec
Throughput:		        0.00 MB/sec
Concurrency:		      254.07
Successful transactions:      160988
Failed transactions:	        1024
Longest transaction:	        3.10
Shortest transaction:	        0.01

unable to write to socket sock.c:733: Broken pipe
socket: read error Connection reset by peer sock.c:635: Connection reset by peer
https://github.com/rust-lang/rust/issues/18847
http://mdba.cn/2015/04/07/%e5%85%b3%e4%ba%8etcp-socket%e5%87%ba%e7%8e%b0%e7%9a%84connection-reset-by-peer%e5%92%8cbroken-pipe/
在高并发下会出现该错误，跟mac电脑有关，试试在linux上测试


siege -c 100 -t 10 http://localhost:8080/aaa/1223/test.html
Transactions:		      258586 hits
Availability:		      100.00 %
Elapsed time:		      599.03 secs
Data transferred:	        2.96 MB
Response time:		        0.23 secs
Transaction rate:	      431.67 trans/sec
Throughput:		        0.00 MB/sec
Concurrency:		       99.94
Successful transactions:      258586
Failed transactions:	           0
Longest transaction:	       20.07
Shortest transaction:	        0.00

*/