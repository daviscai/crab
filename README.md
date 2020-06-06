## crab a mvc web framework

基于 actix-web 搭建的 MVC 框架，开箱即用

```
// Rust crab 1.0 ， 目标：基础功能完善，可用于常见的云端API接口业务，使用门槛低，安全，高性能，低时延(<50ms)
// 1、支持 redis 集群和连接池，通过配置完成开箱即用 [done]
// 2、支持 mysql 连接池，开箱即用                [done]
// 3、支持日志功能                              [done]
// 4、支持 https                               [done]
// 5、友好的路由配置                            [done]
// 6、基于mvc模式                              [done]
// 7、支持html/js/css等前端页面渲染              [done]
// 8、Yew                                     [done]
// 9、性能优异，评测数据，                       [done]
//    测试：写日志、读写数据库各1次、读写redis缓存各1次、路由解析、json编解码 
//    测试环境：MacBook Pro ，8 GB内存，Intel Core i5 处理器，macOS: 10.14.6 
//    测试结果：
//      1、并发100, 持续10分钟，258586次请求100%成功，平均时延230ms;
//      2、并发10, 持续10分钟，231866次请求100%成功，平均时延30ms;
//
// 10、单元测试覆盖率100%，完善的文档              [todo]
//
```

框架图：
```
//
// Rust wasm             Rust app server(mvc-rs)
// in browser <- REST -> |-------------------------------------|
//                       | HTTP Server -- actix-web            |
//  |                    |     |                               |
// Yew                   | Diesel (ORM) -> Mysql / PostgreSQL  |
//                       |-------------------------------------|
// 
```

### 使用方法

目录结构：
```
crab
    |- assets/  
        |- images/  
        |- js       // 前端编译后代码
    |- log/         // 日志存放目录
    |- static/      // 前端静态页面
        |- hi.html
        |- test_yew.html
        |- js/
            |- main.js          // 前端脚本源码
    |- src/                     // 云端后端服务
        |- config/
            |- locale/          // 本地化多国语言配置
                |- en.json
                |- zh-CN.json
            |- app.toml         // 应用配置，包括 http, mysql , redis 等
            |- router.rs        // 路由配置
        |- controllers/         // 业务控制层，负责业务逻辑实现
            |- app/
                |- home.rs
        |- core/                // 核心模块，业务应用一般不需要修改
            |- app.rs
            |- db.rs
            |- i18n.rs
            |- redis.rs
            |- util.rs
        |- models/              // 业务数据层，负责业务数据访问封装
            |- post.rs
            |- schema.rs
        |- view/                // 视图层，负责前端页面渲染处理，前端也采用rust开发
            |- lib.rs           // 前端业务代码入口
            |- app.rs           // 前端业务模块代码
            |- Cargo.toml       // 前端业务的编译配置，执行 wasm-pack build 后会产出 pkg和target 两个目录和相关js和wasm文件
        |- lib.rs
        |- main.rs              // 入口
    |- Cargo.toml               // 项目配置
    |- log4rs.yaml              // log 配置
    |- rollup.config.js         // rollup 前端打包配置
```


### cargo run watch 

修改代码后自动触发重新编译运行，需安装 cargo-watch   
cargo install cargo-watch 


### 快速开始
后端代码修改保存后自动重新编译运行
```
cargo watch -x run 
```

浏览器打开 ： `http://localhost:8080/hi.html` 


### Yew 前端开发快速开始

1、安装 wasm-pack 并编译 .wasm 文件
```
cargo install wasm-pack
wasm-pack build --target web src/view
cp src/view/pkg/crab_bg.wasm assets/pkg/
```

2、安装 rollup 前端编译工具，并编译最终 js 文件
```
brew install npm
npm install --global rollup
rollup -c  # 或者 rollup --config rollup.config.js
```

#### cargo watch 

前端代码修改保存后自动编译生效

```
cargo watch -s 'wasm-pack build --target web src/view' -s 'cp src/view/pkg/crab_bg.wasm assets/pkg/' -s 'rollup -c' -s 'cargo run' -w src
```

浏览器打开 ： `http://localhost:8080/test_yew.html` 