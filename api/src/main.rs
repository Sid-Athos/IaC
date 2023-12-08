mod database;
mod service;
mod domain;
mod entrypoint;
mod middlewares;

use std::env;
use axum::{ Router};
use std::net::SocketAddr;



use dotenv::dotenv;
use tower_http::cors::CorsLayer;


use utoipa::{ OpenApi};


use utoipa_swagger_ui::SwaggerUi;

use crate::middlewares::{tracing::init_tracer, cors_layer::init_cors_layer};
use crate::database::init::init_db;
use crate::swagger_security::SecurityAddon;

use crate::entrypoint::ranking::ranking_router::ranking_routes;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::entrypoint::user::route::response::session_response::SessionResponse;


use crate::entrypoint::ranking::route::response::ranking_response::RankingResponse;
use crate::entrypoint::admin::admin_router::admin_routes;
use crate::entrypoint::friend_list::friend_list_router::friend_list_routes;
use crate::entrypoint::user::user_router::user_routes;
use crate::entrypoint::user::route::request::create_user_request::CreateUserRequest;
use crate::entrypoint::user::route::request::update_user_request::UpdateUserRequest;
use crate::entrypoint::user::route::request::change_password_request::ChangePasswordRequest;
use crate::entrypoint::user::route::request::login_request::LoginRequest;
use crate::middlewares::swagger_security;


#[tokio::main]
async fn main() {

    dotenv().ok();
    init_tracer();
    let pool = init_db().await.unwrap();

    let _cors = init_cors_layer();

    println!(env!("CARGO_MANIFEST_DIR"));
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge( user_routes(pool.clone()))
        .merge( admin_routes(pool.clone()))
        .merge( ranking_routes(pool.clone()))
        .merge(friend_list_routes(pool.clone()))
        .layer(CorsLayer::permissive());

    let addr : SocketAddr = (&env::var("SERVER").unwrap()).parse().expect("Not a socket address");

    tracing::info!("listening on {:?}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}


#[derive(OpenApi)]
#[openapi(
paths(
entrypoint::user::route::get_by_id::user_get,
entrypoint::user::route::create::user_create,
entrypoint::user::route::login::user_login,
entrypoint::user::route::logout::user_logout,
entrypoint::admin::route::add_experience::add_experience,
entrypoint::user::route::me::me,
entrypoint::user::route::search::search_user,
entrypoint::user::route::search::get_other_players,
entrypoint::user::route::delete::delete_user,
entrypoint::user::route::update::update_user,
entrypoint::user::route::change_password::change_password,
entrypoint::admin::route::get_all::get_all,
entrypoint::admin::route::delete::delete_user,
entrypoint::admin::route::give_admin_role::give_admin_role,
entrypoint::admin::route::add_experience::add_experience,
entrypoint::admin::route::remove_admin_role::remove_admin_role,
entrypoint::admin::route::update_user::update_user,
entrypoint::friend_list::route::create::friend_list_create,
entrypoint::friend_list::route::delete_friend::delete_friend,
entrypoint::friend_list::route::accept_friend_request::accept_friend_request,
entrypoint::friend_list::route::show_friend_list::show_friend_list,
entrypoint::friend_list::route::show_friend_request::show_friend_request,
entrypoint::ranking::route::init_ranking::init_ranking,
entrypoint::ranking::route::update_ranking::update_ranking,
entrypoint::ranking::route::get_ranking_by_friend::get_ranking_by_friend,
entrypoint::ranking::route::get_ranking_by_game::get_ranking_by_game_id,
entrypoint::ranking::route::get_ranking_by_user::get_ranking_by_user_id,
entrypoint::ranking::route::get_ranking_by_user_and_game::get_ranking_by_user_id_and_game_id,
entrypoint::ranking::route::delete_by_game::delete_by_game,
entrypoint::ranking::route::delete_by_user::delete_by_user
),
modifiers(&SecurityAddon),
components(
schemas(UserResponse),
schemas(SessionResponse),
schemas(LoginRequest),
schemas(CreateUserRequest),
schemas(ChangePasswordRequest),
schemas(UpdateUserRequest),

),
)]
struct ApiDoc;


