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
    clippy::module_name_repetitions, // repetition of module name in a struct name is not a big deal
    clippy::multiple_crate_versions, // multi-version dependency crates is not able to fix
    clippy::panic, // allow debug_assert, panic in production code
    clippy::unreachable, // Use `unreachable!` instead of `panic!` when impossible cases occur
    clippy::missing_errors_doc, // TODO: add error docs
    clippy::exhaustive_structs,
    clippy::exhaustive_enums,
    clippy::missing_panics_doc, // TODO: add panic docs
    clippy::panic_in_result_fn,
    clippy::single_char_lifetime_names,
    clippy::separated_literal_suffix, // conflict with unseparated_literal_suffix
    clippy::undocumented_unsafe_blocks, // TODO: add safety comment
    clippy::missing_safety_doc, // TODO: add safety comment
    clippy::shadow_unrelated, // it’s a common pattern in Rust code
    clippy::shadow_reuse, // it’s a common pattern in Rust code
    clippy::shadow_same, // it’s a common pattern in Rust code
    clippy::same_name_method, // Skip for protobuf generated code
    clippy::mod_module_files, // TODO: fix code structure to pass this lint
    clippy::std_instead_of_core, // Cause false positive in src/common/error.rs
    clippy::std_instead_of_alloc,
    clippy::pub_use, // pub use mod::item as new_name is common in Rust
    clippy::missing_trait_methods, // TODO: fix this
    clippy::arithmetic_side_effects, // TODO: fix this
    clippy::use_debug, // Allow debug print
    clippy::print_stdout, // Allow println!
    clippy::question_mark_used, // Allow ? operator, it’s a common pattern in Rust code
    clippy::absolute_paths, // Allow use through absolute paths, like `std::env::current_dir`
    clippy::multiple_unsafe_ops_per_block, // Mainly caused by `etcd_delegate`, will remove later
    clippy::ref_patterns, // Allow Some(ref x)
    clippy::single_call_fn, // Allow function is called only once
    clippy::pub_with_shorthand, // Allow pub(super)
    clippy::min_ident_chars, // Allow Err(e)
    clippy::missing_assert_message, // Allow assert! without message, mainly in test code
    clippy::impl_trait_in_params, // Allow impl AsRef<Path>, it's common in Rust
    clippy::module_inception, // We consider mod.rs as a declaration file
    clippy::semicolon_outside_block, // We need to choose between this and `semicolon_inside_block`, we choose outside
    clippy::similar_names, // Allow similar names, due to the existence of uid and gid
    clippy::missing_inline_in_public_items, // TODO: allow missing inline in public items, will be update after PR 549 is merged
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
)]

pub mod async_fuse;
pub mod common;
/// Configurations
pub mod config;
pub mod distribute_kv_cache;
pub mod fs;
pub mod metrics;
pub mod new_storage;
