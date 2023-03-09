use axum::{debug_handler, extract::{Query, Path}, routing::get, Router, Server};
use serde::{Deserialize, Serialize};

use crate::{
    all::{get_frozen, get_netkans},
    schemas::{frozen::FrozenSchema, netkan::NetKANSchema}, resolver::{id::resolve_mod_by_id, kref::resolve_kref},
};

pub async fn index() -> &'static str {
    return "Hello, world!";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryParameters {
    pub frozen: Option<bool>,
    pub live: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenQueryParameters {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live: Option<Vec<NetKANSchema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen: Option<Vec<FrozenSchema>>,
}

impl Default for QueryResponse {
    fn default() -> Self {
        return Self {
            frozen: None,
            live: None,
        };
    }
}

#[debug_handler]
pub async fn query(Query(params): Query<QueryParameters>) -> String {
    let mut live = true;
    let mut frozen = false;

    if let Some(p_frozen) = params.frozen {
        frozen = p_frozen;
    }

    if let Some(p_live) = params.live {
        live = p_live;
    }

    let mut resp = QueryResponse::default();

    if live {
        resp.live = Some(get_netkans().await);
    }

    if frozen {
        resp.frozen = Some(get_frozen().await);
    }

    return serde_json::to_string(&resp).unwrap();
}

#[debug_handler]
pub async fn get_kref(Path(mod_id): Path<String>, Query(params): Query<TokenQueryParameters>) -> String {
    let kref = resolve_mod_by_id(mod_id).await.unwrap();

    return resolve_kref(kref, params.token).await.unwrap();
}

pub async fn run_server() {
    let router = Router::new();

    let router = router
        .route("/", get(index))
        .route("/query", get(query))
        .route("/kref/:mod_id", get(get_kref));

    let app = Server::bind(&"0.0.0.0:4000".parse().unwrap());
    let server = app.serve(router.into_make_service());

    println!("Serving on 0.0.0.0:4000!");

    server.await.unwrap();
}
