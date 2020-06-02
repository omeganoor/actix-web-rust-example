use crate::api::*;
use actix_web::web;

mod constant;
mod controller;

pub fn user_config_services(cfg: &mut web::ServiceConfig) {
    info!("Configurating user routes...");
    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
            .service(
                web::scope(constant::api_path::USER)
                    .service(
                        web::resource("")
                            .route(web::get().to(user_controller::find_all))
                            .route(web::post().to(user_controller::insert))
                    )
            )
    );
}