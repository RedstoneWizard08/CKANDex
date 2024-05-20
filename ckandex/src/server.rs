use std::{env, net::{IpAddr, SocketAddr}};

use axum::{
    debug_handler, extract::{Path, Query, State}, response::Response, routing::get, serve, Router
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::{kref::resolve_kref, CacheClient, IdFilter, NameFilter, QueryBuilder};

pub async fn index() -> &'static str {
    return "Hello, world!";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryParameters {
    pub filter: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenQueryParameters {
    pub token: String,
}

#[debug_handler]
pub async fn query(
    Query(params): Query<QueryParameters>,
    State(cache): State<CacheClient>,
) -> String {
    let mut query = QueryBuilder::new();

    if let Some(filter) = params.filter {
        query.add(NameFilter::new(filter));
    }

    let query = query.build();
    let resp = query.execute(&cache);

    return serde_json::to_string(&resp).unwrap();
}

#[debug_handler]
pub async fn get_kref(
    Path(mod_id): Path<String>,
    Query(params): Query<TokenQueryParameters>,
    State(cache): State<CacheClient>,
) -> Response<String> {
    let mut query = QueryBuilder::new();
    let query = query.add(IdFilter::new(mod_id)).build();

    let mods = query.execute(&cache);
    let item = mods.first();

    if let Some(item) = item {
        let resolved = resolve_kref(item.kref, params.token).await.unwrap();
        let mut resp = Response::new(resolved);

        *resp.status_mut() = StatusCode::OK;

        return resp;
    }

    let mut resp = Response::new("Unknown mod!".to_string());

    *resp.status_mut() = StatusCode::BAD_REQUEST;

    return resp;
}

#[debug_handler]
pub async fn get_kref_env(
    Path(mod_id): Path<String>,
    State(cache): State<CacheClient>,
) -> Response<String> {
    let mut query = QueryBuilder::new();
    let query = query.add(IdFilter::new(mod_id)).build();

    let mods = query.execute(&cache);
    let item = mods.first();

    let token = env::var("GITHUB_TOKEN").unwrap();

    if let Some(item) = item {
        let resolved = resolve_kref(item.kref, token).await.unwrap();
        let mut resp = Response::new(resolved);

        *resp.status_mut() = StatusCode::OK;

        return resp;
    }

    let mut resp = Response::new("Unknown mod!".to_string());

    *resp.status_mut() = StatusCode::BAD_REQUEST;

    return resp;
}

pub async fn run_server(dir: String) {
    let mut cache = CacheClient::new(dir);

    cache.update_cache().await.unwrap();

    let router = Router::new();

    let addr = SocketAddr::from(("0.0.0.0".parse::<IpAddr>().unwrap(), 4000));
    let listener = TcpListener::bind(&addr).await.unwrap();

    if env::var("GITHUB_TOKEN").is_ok() {
        let router = router
            .route("/", get(index))
            .route("/mods", get(query))
            .route("/download/:mod_id", get(get_kref_env))
            .with_state(cache).into_make_service_with_connect_info::<SocketAddr>();

        println!("Serving on 0.0.0.0:4000!");

        serve(listener, router).await.unwrap();
    } else {
        let router = router
            .route("/", get(index))
            .route("/mods", get(query))
            .route("/download/:mod_id", get(get_kref))
            .with_state(cache);

        println!("Serving on 0.0.0.0:4000!");

        serve(listener, router).await.unwrap();
    }
}
