// The following lints are newly fired by the clippy of Rust 1.88
// (the toolchain pinned by rust-toolchain.toml), TODO: fix them
#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::unused_trait_names,
    clippy::doc_markdown,
    clippy::result_large_err,
    clippy::unnecessary_debug_formatting,
    clippy::field_scoped_visibility_modifiers,
    clippy::integer_division_remainder_used,
    clippy::renamed_function_params,
    clippy::get_first,
    clippy::unnecessary_semicolon,
    clippy::uninlined_format_args,
    clippy::unnecessary_get_then_check,
    clippy::iter_over_hash_type,
    clippy::non_std_lazy_statics,
    clippy::borrow_as_ptr,
    clippy::return_and_then,
    clippy::legacy_numeric_constants,
    clippy::needless_continue,
    clippy::multiple_bound_locations,
    clippy::manual_repeat_n,
    clippy::manual_unwrap_or_default,
    clippy::assigning_clones,
    clippy::new_without_default,
    clippy::io_other_error
)]

use std::{str::FromStr, sync::Arc};

use clap::Parser;
use datenlord::{
    common::{
        error::{DatenLordError, DatenLordResult},
        logger::{init_logger, LogRole},
        task_manager::{TaskName, TASK_MANAGER},
    },
    distribute_kv_cache::{
        cluster::{
            cluster_manager::ClusterManager,
            node::{Node, NodeStatus},
        },
        manager::KVCacheHandler,
        rpc::{common::ServerTimeoutOptions, server::RpcServer, workerpool::WorkerPool},
        server_cache::manager::{IndexManager, KVBlockManager},
    },
    fs::kv_engine::{etcd_impl::EtcdKVEngine, KVEngine},
    // metrics,
};
use tracing::{error, info, level_filters::LevelFilter};

/// KV cache server config
#[derive(Debug, Parser)]
#[clap(author,version,about,long_about=None)]
pub struct KVCacheServerConfig {
    /// Log level
    #[clap(
        short = 'l',
        long = "log-level",
        value_name = "LEVEL",
        default_value = "error"
    )]
    log_level: String,
    /// IP
    #[clap(
        short = 'i',
        long = "ip",
        value_name = "IP",
        default_value = "127.0.0.1"
    )]
    ip: String,
    /// Port
    #[clap(
        short = 'p',
        long = "port",
        value_name = "PORT",
        default_value = "2789"
    )]
    port: u16,
    /// ETCD endpoint
    #[clap(
        short = 'e',
        long = "etcd-endpoint",
        value_name = "ENDPOINT",
        default_value = "localhost:2379"
    )]
    etcd_endpoint: String,
}

#[tokio::main]
async fn main() -> DatenLordResult<()> {
    let config = KVCacheServerConfig::parse();
    init_logger(
        LogRole::Cache,
        LevelFilter::from_str(config.log_level.as_str()).map_err(|e| {
            DatenLordError::ArgumentInvalid {
                context: vec![format!("log level {} is invalid: {}", config.log_level, e)],
            }
        })?,
    );

    let ip = config.ip;
    let port = config.port;
    let addr = format!("{}:{}", ip, port);
    info!("KV cache server started at {}", addr);

    let etcd_endpoint = config.etcd_endpoint;
    let client = EtcdKVEngine::new(vec![etcd_endpoint.to_owned()])
        .await
        .expect("Failed to create EtcdKVEngine");
    let client = Arc::new(client);
    info!("Connected to etcd server at {}", etcd_endpoint);

    let node = Node::new(ip.to_owned(), port, 1, NodeStatus::Initializing);
    let cluster_manager = Arc::new(ClusterManager::new(client, node));
    let cluster_manager_clone: Arc<ClusterManager> = Arc::clone(&cluster_manager);
    TASK_MANAGER
        .spawn(TaskName::Rpc, |_token| async move {
            if let Err(e) = cluster_manager_clone.run().await {
                error!("Failed to start cluster manager: {:?}", e);
            }
        })
        .await
        .map_err(|e| DatenLordError::DistributeCacheManagerErr {
            context: vec![format!("Failed to start cluster manager: {:?}", e)],
        })?;

    let cache_manager = Arc::new(KVBlockManager::default());
    let index_manager = Arc::new(IndexManager::<u32>::new());

    let pool = Arc::new(WorkerPool::new(5, 5));
    let handler = KVCacheHandler::new(Arc::clone(&pool), cache_manager, index_manager);
    let mut server = RpcServer::new(&ServerTimeoutOptions::default(), 5, 5, handler);
    match server.listen(&addr).await {
        Ok(()) => {
            info!("KV cache server started successfully");
        }
        Err(err) => {
            panic!("Failed to start kv cache server: {:?}", err);
        }
    }

    tokio::signal::ctrl_c().await?;
    info!("KV cache server stopped");
    Ok(())
}
