use crate::controller::*;
use crate::config::app;
use crate::constant::*;

use actix_web::web;

pub fn user_config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(api_path::API)
            .service(ping_controller::ping)
            .service(app::index)
            // .service(app::index_path)
            // .service(user_controller::find_all)
            .service(web::scope(api_path::USER)
                .service(user_controller::find_all)
                .service(user_controller::create_user)
                .service(user_controller::find_by_id)
            )
            // .service(
            //     web::scope(api_path::USER)
            //         .service(
            //             web::resource("")
            //                 .route(web::get().to(user_controller::find_all))
            //                 // .route(web::post().to(user_controller::insert))
            //         )
            // )
    );
}