use axum::{middleware, Router};
use bb8::Pool;
use axum::routing::{get, delete, put, post};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::middleware::is_logged::is_logged;
use crate::entrypoint::middleware::is_logged_admin::is_logged_admin;
use crate::entrypoint::ranking::route::delete_by_game::delete_by_game;
use crate::entrypoint::ranking::route::delete_by_user::delete_by_user;
use crate::entrypoint::ranking::route::get_ranking_by_friend::get_ranking_by_friend;
use crate::entrypoint::ranking::route::get_ranking_by_game::get_ranking_by_game_id;
use crate::entrypoint::ranking::route::get_ranking_by_user::get_ranking_by_user_id;
use crate::entrypoint::ranking::route::get_all_rankings_by_game::get_all_ranking_by_games;
use crate::entrypoint::ranking::route::get_ranking_by_user_and_game::get_ranking_by_user_id_and_game_id;
use crate::entrypoint::ranking::route::init_ranking::init_ranking;
use crate::entrypoint::ranking::route::update_ranking::update_ranking;
use crate::middlewares::swagger_security::check_api_key;

pub fn ranking_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/ranking", post(init_ranking).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/ranking", put(update_ranking).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/ranking/user/:user_id", delete(delete_by_user).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/ranking/game/:game_id", delete(delete_by_game).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/ranking/user/:user_id", get(get_ranking_by_user_id).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/ranking/user/:user_id/:game_id", get(get_ranking_by_user_id_and_game_id).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/ranking/friend/:game_id", get(get_ranking_by_friend).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/ranking/game/:game_id", get(get_ranking_by_game_id).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/ranking/games", get(get_all_ranking_by_games))
        .with_state(pool)

}