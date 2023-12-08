use axum::{middleware, Router};
use axum::routing::{get, post, delete, put};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::middleware::is_logged::{is_logged};
use crate::entrypoint::user::route::add_experience::add_experience;
use crate::entrypoint::user::route::change_password::change_password;
use crate::entrypoint::user::route::create::user_create;
use crate::entrypoint::user::route::delete::delete_user;
use crate::entrypoint::user::route::get_by_id::user_get;
use crate::entrypoint::user::route::login::user_login;
use crate::entrypoint::user::route::logout::user_logout;
use crate::entrypoint::user::route::me::me;
use crate::entrypoint::user::route::search::{get_other_players, search_user};

use crate::entrypoint::user::route::update::update_user;
use crate::middlewares::swagger_security::check_api_key;

pub fn user_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/user/:user_id", get(user_get).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/user", post(user_create))
        .route("/user/login", post(user_login))
        .route("/user/logout", post(user_logout).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/user/", delete(delete_user).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/user/me", get(me).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/user/change_password", put(change_password).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/user", put(update_user).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/user/search/:pseudo", get(search_user).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/user/other_players/:user_id", get(get_other_players).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .layer(middleware::from_fn(check_api_key))
        .with_state(pool)

}
