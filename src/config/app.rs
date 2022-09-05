// use crate::api::names_controller;
// use actix_web::web;

// pub fn config_services(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/api/v1").service(
//             web::scope("/names")
//                 .service(web::resource("").route(web::get().to(names_controller::get_all))),
//         ),
//     );
// }
