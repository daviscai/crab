use actix_web::{web};
use actix_files as fs;

use crate::controllers::app::home;

pub fn routes(app: &mut web::ServiceConfig) {
    app
    .service(web::resource("/").to(home::index))
    .service(web::resource("/{username}/{id}/test.html").to(home::test))
    // URL:/images ，访问 assets/images/ 目录
    .service(fs::Files::new("/images", "assets/images/").show_files_listing())
    // URL:/js ，访问 assets/js/ 目录
    .service(fs::Files::new("/js", "assets/js/").show_files_listing())
    // URL:/pkg ，访问 assets/pkg/ 目录
    .service(fs::Files::new("/pkg", "assets/pkg/").show_files_listing())
    // static files , URL: /ws.html, 访问 static/ 目录
    .service(fs::Files::new("/", "static/").index_file("hi.html"))
    .service(fs::Files::new("/", "static/").index_file("test_yew.html"))
    // .service(
    //     web::scope("/api")
    //         // User routes
    //         .service(web::resource("users")
    //             .route(web::post().to(home::index))
    //     ),
    // )
    // .service(
    //     web::scope("/ws")
    //         .service(web::resource("chat")
    //             .route(web::post().to(home::index))
    //     ),
    // )
    ;
}