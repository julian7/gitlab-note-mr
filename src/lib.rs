mod handlers;
mod settings;

use log::*;
use settings::Settings;
use actix_web::{middleware, App, HttpServer};
use actix_web::web::Data;
use actix_web::dev::Server;

pub fn app(configfile: &str) -> Server {
    let app_config = Settings::read(configfile).unwrap();
    let loglevel = app_config.loglevel();

    stderrlog::new()
        .module(module_path!())
        .module("actix_web::middleware")
        .verbosity(loglevel)
        .init()
        .unwrap();

    let bind = app_config.bind.clone();
    trace!("setting loglevel to {}", loglevel);
    info!("listening on {}", bind);
    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .configure(handlers::configure)
        .app_data(Data::new(app_config.clone()))
    })
    .bind(bind)
        .unwrap()
        .run()
}
