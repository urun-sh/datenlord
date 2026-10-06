//! `DatenLord`

#![deny(
    // The following are allowed by default lints according to
    // https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
    anonymous_parameters,
    bare_trait_objects,
    // box_pointers,
    // elided_lifetimes_in_paths, // allow anonymous lifetime
    // missing_copy_implementations, // Copy may cause unnecessary memory copy
    missing_debug_implementations,
    missing_docs, // TODO: add documents
    single_use_lifetimes, // TODO: fix lifetime names only used once
    trivial_casts, // TODO: remove trivial casts in code
    trivial_numeric_casts,
    // unreachable_pub, allow clippy::redundant_pub_crate lint instead
    // unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    // unused_results, // TODO: fix unused results
    variant_size_differences,

    warnings, // treat all wanings as errors

    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::cargo
)]
#![allow(
    // Some explicitly allowed Clippy lints, must have clear reason to allow
    clippy::blanket_clippy_restriction_lints, // allow clippy::restriction
    clippy::implicit_return, // actually omitting the return keyword is idiomatic Rust code
    clippy::module_name_repetitions, // repeation of module name in a struct name is not big deal
    clippy::multiple_crate_versions, // multi-version dependency crates is not able to fix
    clippy::panic, // allow debug_assert, panic in production code
    clippy::unreachable,  // Use `unreachable!` instead of `panic!` when impossible cases occurs
    // clippy::panic_in_result_fn,
    clippy::missing_errors_doc, // TODO: add error docs
    clippy::exhaustive_structs,
    clippy::exhaustive_enums,
    clippy::missing_panics_doc, // TODO: add panic docs
    clippy::panic_in_result_fn,
    clippy::single_char_lifetime_names,
    clippy::separated_literal_suffix, // conflict with unseparated_literal_suffix
    clippy::undocumented_unsafe_blocks, // TODO: add safety comment
    clippy::missing_safety_doc, // TODO: add safety comment
    clippy::shadow_unrelated, //it’s a common pattern in Rust code
    clippy::shadow_reuse, //it’s a common pattern in Rust code
    clippy::shadow_same, //it’s a common pattern in Rust code
    clippy::same_name_method, // Skip for protobuf generated code
    clippy::mod_module_files, // TODO: fix code structure to pass this lint
    clippy::std_instead_of_core, // Cause false positive in src/common/error.rs
    clippy::std_instead_of_alloc,
    clippy::pub_use, // TODO: fix this
    clippy::missing_trait_methods, // TODO: fix this
    clippy::arithmetic_side_effects, // TODO: fix this
    clippy::use_debug, // Allow debug print
    clippy::print_stdout, // Allow println!
    clippy::question_mark_used, // Allow ? operator
    clippy::absolute_paths,   // Allow use through absolute paths,like `std::env::current_dir`
    clippy::ref_patterns,    // Allow Some(ref x)
    clippy::single_call_fn,  // Allow function is called only once
    clippy::pub_with_shorthand,  // Allow pub(super)
    clippy::min_ident_chars,  // Allow Err(e)
    clippy::multiple_unsafe_ops_per_block, // Mainly caused by `etcd_delegate`, will remove later
    clippy::impl_trait_in_params,  // Allow impl AsRef<Path>, it's common in Rust
    clippy::missing_assert_message, // Allow assert! without message, mainly in test code
    clippy::semicolon_outside_block, // We need to choose between this and `semicolon_inside_block`, we choose outside
    clippy::similar_names, // Allow similar names, due to the existence of uid and gid
    // The following lints are newly fired by the clippy of Rust 1.88
    // (the toolchain pinned by rust-toolchain.toml), TODO: fix them
    clippy::arbitrary_source_item_ordering, // TODO: reorder source items alphabetically
    clippy::allow_attributes, // TODO: use #[expect] instead of #[allow]
    clippy::allow_attributes_without_reason, // TODO: add reason to #[allow] attributes
    clippy::unused_trait_names, // TODO: use `use Trait as _` for trait-only imports
    clippy::doc_markdown, // TODO: fix doc markdown
    clippy::redundant_test_prefix, // TODO: rename test functions without `test_` prefix
    clippy::integer_division_remainder_used, // TODO: use checked arithmetic for division
    clippy::result_large_err, // TODO: box large error variants
    clippy::field_scoped_visibility_modifiers, // TODO: avoid pub(super) on fields
    clippy::renamed_function_params, // TODO: rename trait impl params to match trait
    clippy::unnecessary_semicolon, // TODO: remove unnecessary semicolons
    clippy::non_std_lazy_statics, // TODO: use std::sync::LazyLock
    clippy::get_first, // TODO: use .first() instead of .get(0)
    clippy::cast_lossless, // TODO: remove lossless casts
    clippy::unnecessary_debug_formatting, // TODO: remove unnecessary debug formatting
    clippy::cast_precision_loss, // TODO: avoid lossy casts
    clippy::needless_continue, // TODO: remove needless continue
    clippy::legacy_numeric_constants, // TODO: use std::f64::consts instead of f64::consts
    clippy::assigning_clones, // TODO: clone_into style is not used in this codebase
    clippy::unnecessary_get_then_check, // TODO: use if let instead of get() + check
    clippy::manual_repeat_n, // TODO: use std::iter::repeat_n
    clippy::iter_over_hash_type, // TODO: avoid iterating over hash maps, output order is random
    clippy::empty_line_after_doc_comments, // TODO: remove empty lines after doc comments
    clippy::borrow_as_ptr, // TODO: use std::ptr::addr_of instead of borrow as ptr
    clippy::used_underscore_items, // TODO: avoid using underscore-prefixed items
    clippy::ignore_without_reason, // TODO: add reason to #[ignore]
    clippy::multiple_bound_locations, // TODO: dedup trait bound locations
    clippy::unused_result_ok, // TODO: use plain expression instead of `.ok()`
    clippy::infinite_loop, // TODO: verify these loops can exit
    clippy::struct_field_names, // TODO: remove type name repetition in field names
    clippy::doc_lazy_continuation, // TODO: fix doc lazy continuation lines
    clippy::empty_docs, // TODO: add doc content or remove empty doc comments
    clippy::single_match_else, // TODO: expand to match with a single arm and else
    clippy::new_without_default, // TODO: implement Default
    clippy::uninlined_format_args, // TODO: inline format args
    clippy::return_and_then, // TODO: avoid return x.then()
    clippy::manual_unwrap_or_default, // TODO: use unwrap_or_default
    clippy::io_other_error, // TODO: use io::Error::other
)]

pub mod async_fuse;
mod common;
mod csi;
pub mod fs;
pub mod new_storage;
pub mod storage;

use std::net::SocketAddr;
use std::sync::Arc;

use async_fuse::AsyncFuseArgs;
use clap::Parser;
use csi::meta_data::MetaData;
use csi::scheduler_extender::SchedulerExtender;
use datenlord::common::task_manager::{self, TaskName, TASK_MANAGER};
use datenlord::config::{InnerConfig, NodeRole};
use datenlord::{config, metrics};
use fs::kv_engine::{KVEngine, KVEngineType};

use crate::common::error::DatenLordResult;
use crate::common::etcd_delegate::EtcdDelegate;
use crate::common::logger::init_logger;

/// Parse config from command line arguments, and return the created `MetaData`
async fn parse_metadata(config: &InnerConfig) -> DatenLordResult<MetaData> {
    let etcd_delegate = EtcdDelegate::new(config.kv_addrs.clone()).await?;
    let worker_port = config.csi_config.worker_port;
    let node_id = config.node_name.clone();
    let ip_address = config.node_ip;
    let mount_dir = config.mount_path.clone();

    csi::build_meta_data(
        worker_port,
        node_id,
        ip_address,
        mount_dir.clone(),
        config.role,
        etcd_delegate,
    )
    .await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arg_conf = config::Config::parse();
    let config = InnerConfig::try_from(config::Config::load_from_args(arg_conf)?)?;

    init_logger(config.role.into(), config.log_level);

    match config.role {
        NodeRole::Node => {
            let metadata = parse_metadata(&config).await?;

            let md = Arc::new(metadata);

            let kv_engine = Arc::new(KVEngineType::new(config.kv_addrs.clone()).await?);
            let node_id = config.node_name.clone();
            let ip_address = config.node_ip;
            let mount_dir = config.mount_path.clone();
            let csi_endpoint = config.csi_config.endpoint.clone();
            let driver_name = config.csi_config.driver_name.clone();

            let worker_server = csi::build_grpc_worker_server(Arc::<MetaData>::clone(&md))?;
            let node_server = csi::build_grpc_node_server(&csi_endpoint, &driver_name, md)?;
            TASK_MANAGER
                .spawn(TaskName::Rpc, |token| {
                    csi::run_grpc_servers(token, vec![worker_server, node_server])
                })
                .await?;

            TASK_MANAGER
                .spawn(TaskName::Metrics, metrics::start_metrics_server)
                .await?;

            let async_args = AsyncFuseArgs {
                node_id,
                ip_address,
                server_port: config.server_port,
                mount_dir: mount_dir.clone(),
                storage_config: config.storage,
            };

            TASK_MANAGER
                .spawn(TaskName::AsyncFuse, |token| async {
                    if let Err(e) = async_fuse::start_async_fuse(kv_engine, async_args, token).await
                    {
                        panic!("failed to start async fuse, error is {e:?}"); // Panic or Error log?
                    }
                })
                .await?;
        }
        NodeRole::Controller => {
            let metadata = parse_metadata(&config).await?;
            let md = Arc::new(metadata);

            let end_point = config.csi_config.endpoint.clone();
            let driver_name = config.csi_config.driver_name.clone();
            let controller_server = csi::build_grpc_controller_server(
                &end_point,
                &driver_name,
                Arc::<MetaData>::clone(&md),
            )?;
            TASK_MANAGER
                .spawn(TaskName::Rpc, |token| {
                    csi::run_grpc_servers(token, vec![controller_server])
                })
                .await?;
        }
        NodeRole::SchedulerExtender => {
            let metadata = parse_metadata(&config).await?;
            let md = Arc::new(metadata);
            let port = config.scheduler_extender_port;
            let ip_address = config.node_ip;

            let scheduler_extender = SchedulerExtender::new(
                Arc::<MetaData>::clone(&md),
                SocketAddr::new(ip_address, port),
            );
            TASK_MANAGER
                .spawn(TaskName::SchedulerExtender, move |token| {
                    scheduler_extender.start(token)
                })
                .await?;
        }
        NodeRole::AsyncFuse => {
            let kv_engine = Arc::new(KVEngineType::new(config.kv_addrs.clone()).await?);
            let node_id = config.node_name.clone();
            let ip_address = config.node_ip;
            let mount_dir = config.mount_path.clone();
            let async_args = AsyncFuseArgs {
                node_id,
                ip_address,
                server_port: config.server_port,
                mount_dir: mount_dir.clone(),
                storage_config: config.storage,
            };

            TASK_MANAGER
                .spawn(TaskName::Metrics, metrics::start_metrics_server)
                .await?;

            TASK_MANAGER
                .spawn(TaskName::AsyncFuse, |token| async {
                    if let Err(e) = async_fuse::start_async_fuse(kv_engine, async_args, token).await
                    {
                        panic!("failed to start async fuse, error is {e:?}"); // Panic or Error log?
                    }
                })
                .await?;
        }
        NodeRole::Cache | NodeRole::SDK => {
            panic!("SDK role is not supported yet");
        }
    }

    task_manager::wait_for_shutdown(&TASK_MANAGER)?.await;
    Ok(())
}
