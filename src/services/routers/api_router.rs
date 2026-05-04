use std::{net::SocketAddr};
use axum::{Router, middleware::from_fn, routing::{get, post}};
use tower_http::cors::{CorsLayer, Any};

use crate::{handlers::api::{account::{account_info_api, block_info_api, create_wallet_api, get_balance_api, 
    pending_api, publish_block_api}, credits::{get_credits_info_api, get_topup_payment_request_api, topup_credits_api}, 
    donate::get_donate_address_api, privacy::privacy_policy, stats::get_stats_api, transaction::{create_transaction_api, get_direct_transaction_status_api, get_transaction_status_api}, websocket::websocket_handler}, 
    middleware::{api_internal_middleware::internal_api_middleware, api_log_middleware::api_log_middleware, api_middleware::{protected_api_middleware, public_api_middleware}}, structs::cache::env::ENV_CACHE};

pub async fn initialize_api_routers() {
    let app = set_routes();
    let env = ENV_CACHE.get().unwrap();
    let listen_addr: SocketAddr = env.http_listen_addr.parse()
        .expect("Invalid HTTP_LISTEN_ADDR format");

    let listener = tokio::net::TcpListener::bind(listen_addr)
        .await
        .expect("Failed to bind to address");


    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

fn set_routes() -> Router {
    let public_api_routes = Router::new()
        .route("/wallet/create", get(create_wallet_api)) 

        .route("/stats", get(get_stats_api))
        .route("/privacy", get(privacy_policy))
        
        .route_layer(from_fn(api_log_middleware))
        .route_layer(from_fn(public_api_middleware));
    

    let protected_api_routes = Router::new()
        .route("/account/info/{address}", get(account_info_api))

        .route("/block/info/{block_hash}", get(block_info_api))
        .route("/block/publish", post(publish_block_api))

        .route("/wallet/balance/{address}", get(get_balance_api))
        .route("/wallet/pending/{address}", get(pending_api))

        .route("/payment/request", post(create_transaction_api))
        .route("/payment/subscribe/{transaction_id}", get(websocket_handler))
        .route("/payment/status/{transaction_id}", get(get_transaction_status_api))

        .route("/credits", get(get_credits_info_api))
        .route("/credits/topup/{credits_amount}", get(get_topup_payment_request_api))
        .route("/credits/topup", post(topup_credits_api))

        .route("/donate/address", get(get_donate_address_api))

        .route_layer(from_fn(api_log_middleware))
        .route_layer(from_fn(protected_api_middleware));

    let internal_api_route = Router::new()
        .route("/internal/payment/status/{transaction_id}", get(get_direct_transaction_status_api))
        .route("/internal/status/{transaction_id}", get(get_transaction_status_api))

        
        .route_layer(from_fn(internal_api_middleware));


    let app = Router::new()
        .merge(public_api_routes)
        .merge(protected_api_routes)
        .merge(internal_api_route)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );

    app
}