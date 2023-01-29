#![warn(clippy::all)]
#![allow(clippy::restriction)]
#![deny(rustc::all)]
#![allow(missing_docs)] // todo
#![allow(box_pointers)]
#![allow(incomplete_features)]
#![warn(broken_intra_doc_links)]
#![warn(private_intra_doc_links)]
#![warn(rustdoc::all)]
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
#![feature(lint_reasons)]
//! A good client for browsing reddit without the cancer

use std::error::Error;

/// Handles browse subcommand
pub mod browse;
/// Handles CLI commands
pub mod command;
/// Handles comments
pub mod comment;
/// Handles posts
pub mod post;
/// Handles settings
pub mod settings;
/// Handles subreddits
pub mod subreddit;
#[cfg(test)]
mod tests;
/// Handles users
pub mod user;

use {
	crate::{
		browse::*,
		command::*,
		settings::*,
	},
	clap::Parser,
	enum_map::enum_map,
	post::get_post_comments,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
	// let user_agent = "";
	// let client_id = "";
	// let client_secret = "";
	// let username = "";
	// let password = "";

	// let client = roux::Reddit::new(user_agent, client_id, client_secret)
	// 	.username(username)
	// 	.password(password)
	// 	.login()
	// 	.await?;

	let args = command::Cli::parse();
	let options = None;
	let response = match args.command
	{
		Command::Browse { object } => Response::Browse(browse(object, options).await),
		Command::Setting { object } => Response::Setting(setting(object)),
		Command::Example {} => Response::Example(example_filter().await),
	};
	println!("{:?}", response);
	Ok(())
}

async fn example_filter() -> String
{
	let policy: SeverityPolicy = enum_map! {
		Severity::Ok => FilterAction::Ignore,
		Severity::Questionable => FilterAction::Tag,
		Severity::Awful => FilterAction::Filter,
		Severity::Scourge => FilterAction::Report,

	};

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
	let new = subreddit.latest(1, None).await;
	let article_id = &new.unwrap().data.children.first().unwrap().data.id.clone();
	get_post_comments(&subreddit, article_id, &subreddit_filter, &policy)
		.await
		.expect("Failed to get comments.")
}
