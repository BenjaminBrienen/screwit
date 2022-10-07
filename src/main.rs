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
#![warn(missing_docs)]
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
#![feature(is_some_with)]
#![feature(unboxed_closures)]
//! A good client for browsing reddit without the cancer

use {
	default::default,
	roux::Subreddit,
	screwit::{
		self,
		print_recent_post_comments,
		Action,
		Policy,
		Severity,
		SubredditFilter,
	},
	std::default,
	tokio,
};

/// .
#[tokio::main]
async fn main()
{
	// Create a policy for actioning on different levels of severity.
	let mut policy: Policy = default();
	_ = policy.insert(Severity::Ok, Action::Ignore);
	_ = policy.insert(Severity::Questionable, Action::Tag);
	_ = policy.insert(Severity::Awful, Action::Tag);
	_ = policy.insert(Severity::Scourge, Action::Filter);
	// Which subreddits to watch out for and how bad each one is considered to be.
	let subreddit_filter = SubredditFilter::new(
		&[
			("conservative".to_string(), Severity::Awful),
			("conspiracy".to_string(), Severity::Awful),
			("sino".to_string(), Severity::Scourge),
			("ConservativeMemes".to_string(), Severity::Awful),
			("conservatives".to_string(), Severity::Awful),
			("republican".to_string(), Severity::Awful),
			("walkaway".to_string(), Severity::Awful),
			("louderwithcrowder".to_string(), Severity::Awful),
			("shitpoliticssays".to_string(), Severity::Awful),
			("progun".to_string(), Severity::Awful),
			("prolife".to_string(), Severity::Awful),
			("gunpolitics".to_string(), Severity::Awful),
			("asktrumpsupporters".to_string(), Severity::Awful),
			("socialjusticeinaction".to_string(), Severity::Awful),
			("goldandblack".to_string(), Severity::Awful),
			("shitstatistssay".to_string(), Severity::Awful),
			("libertarian".to_string(), Severity::Awful),
			("libertarianmeme".to_string(), Severity::Awful),
			("coronaviruscirclejerk".to_string(), Severity::Awful),
			("anarcho_capitalism".to_string(), Severity::Awful),
			("firearms".to_string(), Severity::Awful),
			("nonewnormal".to_string(), Severity::Awful),
			("theleftcantmeme".to_string(), Severity::Awful),
			("jordanpeterson".to_string(), Severity::Awful),
			("lockdownskepticism".to_string(), Severity::Awful),
			("centrist".to_string(), Severity::Awful),
			("actualpublicfreakouts".to_string(), Severity::Awful),
			("protectandserve".to_string(), Severity::Awful),
			("kotakuinaction".to_string(), Severity::Awful),
			("moderatepolitics".to_string(), Severity::Awful),
			("conspiracy_commons".to_string(), Severity::Awful),
			("trueunpopularopinion".to_string(), Severity::Awful),
			("joerogan".to_string(), Severity::Awful),
			("mensrights".to_string(), Severity::Awful),
			("memes".to_string(), Severity::Ok),
		],
		policy,
	);
	print_recent_post_comments(&Subreddit::new("gaming"), &subreddit_filter).await;
}
