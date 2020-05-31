## crab a mvc web framework

基于 actix-web 搭建的 MVC 框架，开箱即用

### 使用方法

目录结构：
```
crab
    |- assets/  
        |- images/
        |- js
    |- log/         // 日志存放目录
    |- static/      // 前端静态页面
        |- hi.html
    |- src/         // 云端后端服务
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
        |- lib.rs
        |- main.rs  // 入口
    |- Cargo.toml   // 项目依赖项配置
    |- log4rs.yaml  // log 配置
```


### cargo run watch 

修改代码后自动触发重新编译运行，需安装 cargo-watch   
cargo install cargo-watch 


### 快速开始
```
cargo watch -x run 
```

浏览器打开 ： `http://localhost:8080/hi.html` 