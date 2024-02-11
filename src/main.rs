// nightly sometimes removes/renames lints
#![cfg_attr(allow_unknown_lints, allow(unknown_lints))]
#![cfg_attr(allow_unknown_lints, allow(renamed_and_removed_lints))]
// enable all rustc's built-in lints
#![deny(
	future_incompatible,
	nonstandard_style,
	rust_2018_compatibility,
	rust_2018_idioms,
	rust_2021_compatibility,
	unused,
	warnings
)]
// rustc's additional allowed by default lints
#![deny(
	absolute_paths_not_starting_with_crate,
	deprecated_in_future,
	elided_lifetimes_in_paths,
	explicit_outlives_requirements,
	ffi_unwind_calls,
	keyword_idents,
	let_underscore_drop,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	missing_debug_implementations,
	non_ascii_idents,
	noop_method_call,
	pointer_structural_match,
	rust_2021_incompatible_closure_captures,
	rust_2021_incompatible_or_patterns,
	rust_2021_prefixes_incompatible_syntax,
	rust_2021_prelude_collisions,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unsafe_code,
	unsafe_op_in_unsafe_fn,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
	unused_lifetimes,
	unused_macro_rules,
	unused_qualifications,
	unused_results,
	unused_tuple_struct_fields,
	variant_size_differences
)]
// enable all of Clippy's lints
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::restriction)]
#![allow(
	clippy::absolute_paths,
	clippy::arithmetic_side_effects,
	clippy::arithmetic_side_effects,
	clippy::as_conversions,
	clippy::blanket_clippy_restriction_lints,
	clippy::bool_to_int_with_if,
	clippy::default_numeric_fallback,
	clippy::else_if_without_else,
	clippy::expect_used,
	clippy::float_arithmetic,
	clippy::implicit_return,
	clippy::indexing_slicing,
	clippy::map_err_ignore,
	clippy::min_ident_chars,
	clippy::missing_docs_in_private_items,
	clippy::missing_trait_methods,
	clippy::module_name_repetitions,
	clippy::needless_raw_string_hashes,
	clippy::needless_raw_strings,
	clippy::new_without_default,
	clippy::non_ascii_literal,
	clippy::option_if_let_else,
	clippy::pattern_type_mismatch,
	clippy::pub_use,
	clippy::pub_with_shorthand,
	clippy::question_mark_used,
	clippy::redundant_closure_call,
	clippy::redundant_closure_for_method_calls,
	clippy::redundant_pub_crate,
	clippy::ref_patterns,
	clippy::self_named_module_files,
	clippy::single_call_fn,
	clippy::std_instead_of_alloc,
	clippy::std_instead_of_core,
	clippy::tabs_in_doc_comments,
	clippy::tests_outside_test_module,
	clippy::too_many_lines,
	clippy::unwrap_used,
	clippy::wildcard_enum_match_arm
)]
#![deny(
	rustdoc::bare_urls,
	rustdoc::broken_intra_doc_links,
	rustdoc::invalid_codeblock_attributes,
	rustdoc::invalid_html_tags,
	rustdoc::private_doc_tests,
	rustdoc::private_intra_doc_links
)]
// allow some things in tests
#![cfg_attr(
	test,
	allow(
		let_underscore_drop,
		clippy::cast_possible_truncation,
		clippy::cognitive_complexity,
		clippy::let_underscore_must_use,
		clippy::let_underscore_untyped,
		clippy::needless_pass_by_value,
		clippy::panic,
		clippy::shadow_reuse,
		clippy::shadow_unrelated,
		clippy::undocumented_unsafe_blocks,
		clippy::unimplemented,
		clippy::unreachable
	)
)]
// allowable upcoming nightly lints
#![cfg_attr(include_nightly_lints, allow(clippy::arc_with_non_send_sync))]
#![allow(unused)]

mod application;
mod arguments;
mod components;
mod config;
mod display;
mod editor;
mod events;
mod exit;
mod git;
mod help;
mod input;
mod license;
mod module;
mod modules;
mod process;
mod runtime;
mod search;
#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod testutil;
mod todo_file;
mod util;
mod version;
mod view;

use std::{env, ffi::OsString};

use crate::{
	arguments::{Args, Mode},
	exit::Exit,
};

#[must_use]
pub fn run(os_args: Vec<OsString>) -> Exit {
	match Args::try_from(os_args) {
		Err(err) => err,
		Ok(args) => {
			match *args.mode() {
				Mode::Help => help::run(),
				Mode::Version => version::run(),
				Mode::License => license::run(),
				Mode::Editor => editor::run(&args),
			}
		},
	}
}

// TODO use the termination trait once rust-lang/rust#43301 is stable
#[allow(clippy::exit, clippy::print_stderr)]
#[cfg(not(tarpaulin_include))]
fn main() {
	let exit = run(env::args_os().skip(1).collect());
	if let Some(message) = exit.get_message().as_ref() {
		eprintln!("{message}");
	}
	std::process::exit(exit.get_status().to_code());
}
