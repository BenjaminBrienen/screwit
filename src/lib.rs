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
//! A good client for browsing reddit without the cancer

use {
	colored::{
		Color,
		ColoredString,
		Colorize,
	},
	default::default,
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
	std::{
		collections::HashMap,
		default,
	},
};

/// .
pub fn iterate_reply(
	item: Vec<BasicThing<Box<CommentData>>>,
	depth: usize,
)
{
	for reply in item
	{
		if let Some(text) = reply.data.body
		{
			println!("{}\n", format_comment(text, depth, true));
		}
		for subreply in reply.data.replies
		{
			if let MaybeReplies::Reply(subreply) = subreply
			{
				iterate_reply(subreply.data.children, depth + 1);
			}
		}
	}
}

/// .
pub fn format_comment(
	input: String,
	depth: usize,
	colorize: bool,
) -> ColoredString
{
	const COLORS: [Color; 6] = [
		colored::Color::Red,
		colored::Color::Yellow,
		colored::Color::Green,
		colored::Color::Cyan,
		colored::Color::Blue,
		colored::Color::Magenta,
	];
	let indent_char = "|";
	let indent = indent_char.repeat(depth);

	let mut rtn = String::new();
	for line in input.split_inclusive("\n")
	{
		rtn.push_str(&indent);
		rtn.push_str(line);
	}
	if colorize
	{
		rtn.color(COLORS[depth % COLORS.len()])
	}
	else
	{
		rtn.normal()
	}
}

/// .
pub async fn get_user_active_subreddits(user: User) -> HashMap<String, i32>
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

/// .
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Severity
{
	/// .
	Scourge,
	/// .
	Awful,
	/// .
	Questionable,
	/// .
	Ok,
}

/// .
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Action
{
	/// .
	Filter,
	/// .
	Tag,
	/// .
	Ignore,
}

/// .
#[derive(Default, Debug)]
pub struct SubredditFilter
{
	/// .
	mapping: HashMap<String, Severity>,
	/// .
	policy:  Policy,
}

/// .
pub type Policy = HashMap<Severity, Action>;

impl SubredditFilter
{
	/// .
	pub fn filter_severity(
		self: &Self,
		subreddits: Vec<String>,
		severity: Severity,
	) -> Vec<String>
	{
		subreddits
			.into_iter()
			.filter(|subreddit| {
				self
					.mapping
					.get(subreddit)
					.is_some_and(|subreddit_severity| -> bool { **subreddit_severity == severity })
			})
			.collect::<Vec<String>>()
	}

	/// .
	pub fn new(
		array_tuple: &[(String, Severity)],
		policy: HashMap<Severity, Action>,
	) -> Self
	{
		let mut map: HashMap<String, Severity> = default();
		for element in array_tuple
		{
			_ = map.insert(element.0.clone(), element.1);
		}
		SubredditFilter { mapping: map, policy }
	}
}

/// .
pub async fn get_user_subreddits_with_severity(
	username: &String,
	bad_subreddit_filter: &SubredditFilter,
	severity: Severity,
) -> Vec<String>
{
	// Get all subreddits a user has affiliated with.
	let user_subreddits = get_user_active_subreddits(User::new(username.as_str())).await;
	bad_subreddit_filter.filter_severity(threshhold_subreddits(user_subreddits, 0).unwrap_or_default(), severity)
}

/// .
pub fn print_subreddits(
	subreddits: Vec<String>,
	color: Color,
)
{
	println!(
		"{}",
		subreddits
			.iter()
			.map(|x| -> String { format!("{}\n", x) })
			.collect::<String>()
			.on_color(color)
	);
}

/// .
pub fn threshhold_subreddits(
	subreddits: HashMap<String, i32>,
	threshhold: i32,
) -> Option<Vec<String>>
{
	let mut rtn: Vec<String> = default();
	for subreddit in subreddits
	{
		if subreddit.1 > threshhold
		{
			rtn.push(subreddit.0.to_string());
		}
	}
	return Some(rtn)
}

/// .
pub async fn print_recent_post_comments(
	subreddit: &Subreddit,
	subreddit_filter: &SubredditFilter,
)
{
	let new = subreddit.hot(1, None).await;
	let article_id = &new.unwrap().data.children.first().unwrap().data.id.clone();
	let article_comments = subreddit.article_comments(article_id, None, None).await;
	println!("{}", article_id);
	for comment in article_comments.expect("msg").data.children
	{
		if let Some(username) = comment.data.author
		{
			println!("{}", username.on_color(Color::Black).color(Color::White));
			print_subreddits(get_user_subreddits_with_severity(&username, subreddit_filter, Severity::Awful).await, Color::BrightRed);
		}
		if let Some(text) = comment.data.body.clone()
		{
			println!("{}\n", format_comment(text, 0, true));
		}
		for reply in comment.data.replies
		{
			if let MaybeReplies::Reply(reply) = reply
			{
				iterate_reply(reply.data.children, 1)
			}
		}
		println!("=====");
	}
}

fn and<'a, T: IntoIterator<Item = &'a bool>>(
	left: T,
	right: T,
) -> Vec<bool>
{
	left.into_iter().zip(right).map(|x| *x.0 && *x.1).collect()
}

fn or<'a, T: IntoIterator<Item = &'a bool>>(
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
		crate::{
			format_comment,
			get_user_active_subreddits,
		},
		roux::User,
		test_case::test_case,
	};

	#[test_case("000111222333444555666777888999888777666555444333222111000", 1, "|0001112223\n|3344455566\n|6777888999\n|8887776665\n|5544433322\n|2111000";           "10 wide, 1 indent level")]
	#[test_case("000111222333444555666777888999888777666555444333222111000", 2, "||0001112223\n||3344455566\n||6777888999\n||8887776665\n||5544433322\n||2111000"; "10 wide, 2 indent level")]
	#[test_case("000111222333444555666777888999888777666555444333222111000", 1, "|00011122233344455566\n|67778889998887776665\n|55444333222111000"; "20 wide, 1 indent level")]
	#[test_case("000111222333444555666777888999888777666555444333222111000", 2, "||00011122233344455566\n||67778889998887776665\n||55444333222111000"; "20 wide, 2 indent level")]
	#[test_case("01234567890123456789", 1, "|0123456789\n|0123456789"; "two tens")]
	fn test_format_string(
		input: &str,
		indentation: usize,
		expected_output: &str,
	)
	{
		let result = format_comment(input.to_string(), indentation, false);
		assert_eq!(result.to_string(), expected_output);
	}

	#[tokio::test]
	async fn test_get_user_active_subreddits()
	{
		let x = get_user_active_subreddits(User::new("ImTheTechn0mancer")).await;
		assert!(x.len() != 0);
	}
}
