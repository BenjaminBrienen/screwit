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
#![feature(unboxed_closures)]
//! A good client for browsing reddit without the cancer

pub mod browse;
pub mod command;
pub mod comment;
pub mod post;
pub mod settings;
pub mod subreddit;
#[cfg(test)]
mod tests;
pub mod user;

use {
	crate::{
		browse::*,
		command::*,
		settings::*,
	},
	clap::Parser,
	enum_map::enum_map,
	regex::Regex,
	roux::Subreddit,
	screwit::{
		self,
		FilterAction,
		RegexPolicy,
		Severity,
		SeverityPolicy,
		SubredditPolicies,
	},
	tokio,
};

/// .
#[tokio::main]
async fn main()
{
	let args = command::Cli::parse();
	// println!("{:?}", args);
	match args.command
	{
		Command::Browse { object } => browse(object),
		Command::Setting { object } => setting(object),
		Command::Test { value } => println!("{:?}", value),
		Command::Example => example_filter().await,
	}
}

async fn example_filter()
{
	// Create a policy for actioning on different levels of severity.
	let policy: SeverityPolicy = enum_map! {
		Severity::Ok => FilterAction::Ignore,
		Severity::Questionable => FilterAction::Tag,
		Severity::Awful => FilterAction::Filter,
		Severity::Scourge => FilterAction::Report,

	};
	// Which subreddits to watch out for and how bad each one is considered to be.
	#[rustfmt::skip]
	let subreddit_filter: SubredditPolicies = vec!
		[
			RegexPolicy { regex: Regex::new(          "conservative").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(            "conspiracy").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(                  "sino").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(     "ConservativeMemes").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(         "conservatives").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(            "republican").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(              "walkaway").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(     "louderwithcrowder").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(      "shitpoliticssays").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(                "progun").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(               "prolife").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(           "gunpolitics").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(    "asktrumpsupporters").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new( "socialjusticeinaction").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(          "goldandblack").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(       "shitstatistssay").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(           "libertarian").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(       "libertarianmeme").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new( "coronaviruscirclejerk").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(    "anarcho_capitalism").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(              "firearms").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(           "nonewnormal").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(       "theleftcantmeme").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(        "jordanpeterson").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(    "lockdownskepticism").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(              "centrist").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new( "actualpublicfreakouts").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(       "protectandserve").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(        "kotakuinaction").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(      "moderatepolitics").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(    "conspiracy_commons").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(  "trueunpopularopinion").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(              "joerogan").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(            "mensrights").expect(""), severity: Severity::Awful },
			RegexPolicy { regex: Regex::new(                 "memes").expect(""), severity: Severity::Ok },
		];
	let subreddit = Subreddit::new("gaming");
	let new = subreddit.hot(1, None).await;
	let article_id = &new.unwrap().data.children.first().unwrap().data.id.clone();
	if let Ok(comments) = crate::post::get_post_comments(&subreddit, article_id, &subreddit_filter, &policy).await
	{
		println!("{}", comments);
	}
}
