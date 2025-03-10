//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::{
    auth::{login, logout},
    health::get_health,
    user::{create_user, delete_user, get_user, get_users, update_user},
    user::{get_wifi /*, update_wifi, get_wifis, update_wifis*/},
};
use crate::middleware::auth::Auth as AuthMiddleware;
use actix_files::Files;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health))
        // /api/v1 routes
        .service(
            web::scope("/api/v1")
                // Lock down routes with AUTH Middleware
                .wrap(AuthMiddleware)
                // AUTH routes
                .service(
                    web::scope("/auth")
                        .route("/login", web::post().to(login))
                        .route("/logout", web::get().to(logout)),
                )
                // USER routes
                .service(
                    web::scope("/user")
                        .route("/{id}", web::get().to(get_user))
                        .route("/{id}", web::put().to(update_user))
                        .route("/{id}", web::delete().to(delete_user))
                        .route("", web::get().to(get_users))
                        .route("", web::post().to(create_user)),
                )
                .service(
                    web::scope("/wifi")
                        .route("/{id}", web::get().to(get_wifi))
                        /*
                        .route("/{id}", web::put().to(update_wifi))
                        .route("", web::get().to(get_wifi))
                        .route("", web::put().to(update_wifis))
                        */
                )
                /*.service(
                    web::scope("/statistics")
                        .route("/{id}", web::get().to(get_statistics))
                        .route("", web::get().to(get_statistics_all))
                )
                .service(
                    web::scope("/management")
                        .route("/{id}", web::get().to(get_topic))
                        .route("/{id}", web::put().to(update_topic))
                        .route("", web::get().to(get_topic))
                        .route("", web::put().to(update_topics)),
                )*/
        )
        // Serve secure static files from the static-private folder
        .service(
            web::scope("/secure").wrap(AuthMiddleware).service(
                Files::new("", "./static-secure")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        )
        // Serve public static files from the static folder
        .service(
            web::scope("").default_service(
                Files::new("", "./static")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        );
}
