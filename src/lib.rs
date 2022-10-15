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
#![feature(is_some_with)]
//! A good client for browsing reddit without the cancer

use {
	async_recursion::async_recursion,
	// colored::{
	// 	Color,
	// 	ColoredString,
	// 	Colorize,
	// },
	default::default,
	enum_map::{
		Enum,
		EnumMap,
	},
	regex::Regex,
	roux::{
		comment::CommentData,
		response::BasicThing,
		util::{
			FeedOption,
			TimePeriod,
		},
		MaybeReplies::{self,},
		Subreddit,
		User,
	},
	roux::{
		response::Listing,
		submission::SubmissionData,
		util::RouxError,
	},
	std::{
		collections::HashMap,
		default,
	},
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

#[async_recursion]
pub async fn iterate_replies(
	replies: &Vec<BasicThing<Box<CommentData>>>,
	depth: usize,
	subreddit_policies: &SubredditPolicies,
	severity_policy: &SeverityPolicy,
) -> String
{
	let mut rtn: String = default();
	for comment in replies
	{
		let mut username_severity = Severity::Ok;
		let mut tagged_subreddits: Vec<String> = vec![];
		if let Some(username) = &comment.data.author
		{
			for subreddit in filter_subreddits_above_karma_threshhold(get_user_participated_subreddits_and_subreddit_karmas(User::new(username.as_str())).await, 0)
			{
				for subreddit_policy in subreddit_policies
				{
					if subreddit_policy.regex.is_match(subreddit.as_str())
					{
						if username_severity < subreddit_policy.severity
						{
							username_severity = subreddit_policy.severity;
						}
						if severity_policy[username_severity] == FilterAction::Tag
						{
							tagged_subreddits.push(subreddit);
							break
						}
					}
				}
			}
			match severity_policy[username_severity]
			{
				FilterAction::Ignore => rtn.push_str(&username),
				FilterAction::Tag => rtn.push_str(&format!("{} {}", username, tagged_subreddits.concat())),
				FilterAction::Filter => (),
				FilterAction::Report => (),
			}
		}
		if let Some(text) = comment.data.body.clone()
		{
			rtn.push_str(&text);
		}
		while let Some(MaybeReplies::Reply(reply)) = comment.data.replies.iter().next()
		{
			rtn.push_str(&iterate_replies(&reply.data.children, depth + 1, subreddit_policies, severity_policy).await);
		}
	}
	rtn
}

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

pub async fn get_user_participated_subreddits_and_subreddit_karmas(user: User) -> HashMap<String, i32>
{
	let mut subreddits: HashMap<String, i32> = HashMap::new();
	let mut after = String::new();
	while let Ok(comments) = user.comments(Some(
			FeedOption::new()
				.after(&after)
				.limit(100)
				.period(TimePeriod::AllTime),
		)).await
		&& let Some(last) = comments.data.after
	{
		after = last;
		for comment in comments.data.children
		{
			if let Some(subreddit) = comment.data.subreddit
			{
				*subreddits.entry(subreddit.to_lowercase()).or_insert(0) += comment.data.score.unwrap_or(0);
			}
		}
	}
	subreddits
}

pub fn filter_subreddits_above_karma_threshhold(
	subreddits: HashMap<String, i32>,
	threshhold: i32,
) -> Vec<String>
{
	let mut rtn: Vec<String> = default();
	for subreddit in subreddits
	{
		if subreddit.1 > threshhold
		{
			rtn.push(subreddit.0.to_string());
		}
	}
	return rtn
}

pub async fn get_posts() -> Result<BasicThing<Listing<BasicThing<SubmissionData>>>, RouxError>
{
	let subreddit = Subreddit::new("gaming");
	subreddit.hot(1, None).await
}

pub async fn get_post_comments(
	subreddit: &Subreddit,
	article_id: &String,
	subreddit_policies: &SubredditPolicies,
	severity_policy: &SeverityPolicy,
) -> Result<String, String>
{
	if let Ok(article_comments) = subreddit.article_comments(article_id, None, None).await
	{
		Ok(
			iterate_replies(
				&article_comments
					.data
					.children
					.into_iter()
					.map(|b| BasicThing { kind: b.kind, data: Box::new(b.data) })
					.collect::<Vec<BasicThing<Box<CommentData>>>>(),
				0,
				subreddit_policies,
				severity_policy,
			)
			.await,
		)
	}
	else
	{
		Err("Error fetching comments".to_string())
	}
}

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

#[cfg(test)]
mod tests
{
	use {
		crate::get_user_participated_subreddits_and_subreddit_karmas,
		roux::User,
		// test_case::test_case,
	};

	// #[test_case("000111222333444555666777888999888777666555444333222111000", 1, "|000111222333444555666777888999888777666555444333222111000";           "1 indent level")]
	// #[test_case("000111222333444555666777888999888777666555444333222111000", 2, "||000111222333444555666777888999888777666555444333222111000"; "2 indent level")]
	// #[test_case("01234567890123456789", 1, "|01234567890123456789"; "two tens")]
	// fn test_format_string(
	// 	input: &str,
	// 	indentation_depth: usize,
	// 	expected_output: &str,
	// )
	// {
	// 	let result = format_comment::<"|">(input.to_string(), indentation_depth);
	// 	assert_eq!(result.to_string(), expected_output);
	// }

	#[tokio::test]
	async fn test_get_user_active_subreddits()
	{
		let x = get_user_participated_subreddits_and_subreddit_karmas(User::new("ImTheTechn0mancer")).await;
		assert!(x.len() != 0);
	}
}
