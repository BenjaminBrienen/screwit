#![warn(clippy::all)]
#![warn(absolute_paths_not_starting_with_crate)]
#![allow(box_pointers)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(fuzzy_provenance_casts)]
#![warn(keyword_idents)]
#![warn(lossy_provenance_casts)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
//#![warn(missing_docs)]
#![warn(must_not_suspend)]
#![warn(non_ascii_idents)]
#![warn(non_exhaustive_omitted_patterns)]
#![warn(noop_method_call)]
#![warn(pointer_structural_match)]
#![warn(rust_2021_incompatible_closure_captures)]
#![warn(rust_2021_incompatible_or_patterns)]
#![warn(rust_2021_prefixes_incompatible_syntax)]
#![warn(rust_2021_prelude_collisions)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unstable_features)]
#![allow(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_macro_rules)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![warn(unused_tuple_struct_fields)]
#![allow(incomplete_features)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(unstable_features)]
#![feature(try_trait_v2)]
#![feature(let_chains)]
#![feature(adt_const_params)]
#![feature(strict_provenance)]
#![feature(must_not_suspend)]
#![feature(round_char_boundary)]
#![feature(non_exhaustive_omitted_patterns_lint)]
#![feature(custom_test_frameworks)]
#![feature(default_free_fn)]
#![feature(anonymous_lifetime_in_impl_trait)]
//! A good client for browsing reddit without the cancer

use {
	enum_map::{
		Enum,
		EnumMap,
	},
	regex::Regex,
};

// const COLORS: [Color; 6] = [
// 	colored::Color::Red,
// 	colored::Color::Yellow,
// 	colored::Color::Green,
// 	colored::Color::Cyan,
// 	colored::Color::Blue,
// 	colored::Color::Magenta,
// ];

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Enum, PartialOrd)]
pub enum Severity
{
	Ok,
	Questionable,
	Awful,
	Scourge,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Enum)]
pub enum FilterAction
{
	Ignore,
	Tag,
	Filter,
	Report,
}

pub type SeverityPolicy = EnumMap<Severity, FilterAction>;

pub trait ContentFilter
{
	fn filter(
		&self,
		content: &str,
	) -> FilterAction;
}

#[derive(Debug, Clone)]
pub struct RegexPolicy
{
	pub regex:    Regex,
	pub severity: Severity,
}

pub type SubredditPolicies = Vec<RegexPolicy>;
pub type CommentPolicies = Vec<RegexPolicy>;
pub type TextPostPolicies = Vec<RegexPolicy>;
pub type LinkPostPolicies = Vec<RegexPolicy>;
pub type UsernamePolicies = Vec<RegexPolicy>;
// type ImagePostPolicies = Vec<>;

// pub fn format_comment<const INDENT: &'static str>(
// 	input: String,
// 	depth: usize,
// ) -> ColoredString
// {
// 	let indentation = INDENT.repeat(depth);

// 	let mut rtn = String::new();
// 	for line in input.split_inclusive("\n")
// 	{
// 		rtn.push_str(&indentation);
// 		rtn.push_str(line);
// 	}
// 	rtn.color(COLORS[depth % COLORS.len()])
// }

pub fn and<'a, T: IntoIterator<Item = &'a bool>>(
	left: T,
	right: T,
) -> Vec<bool>
{
	left.into_iter().zip(right).map(|x| *x.0 && *x.1).collect()
}

pub fn or<'a, T: IntoIterator<Item = &'a bool>>(
	left: T,
	right: T,
) -> Vec<bool>
{
	left.into_iter().zip(right).map(|x| *x.0 || *x.1).collect()
}
