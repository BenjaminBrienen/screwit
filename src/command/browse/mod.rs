pub mod comment;
pub mod submission;
pub mod subreddit;
pub mod user;

use {
	clap::Subcommand,
	roux::{
		comment::CommentData,
		submission::SubmissionData,
		subreddit::response::SubredditData,
		util::FeedOption,
		Submissions,
		User,
	},
	subreddit::{
		BrowseSubreddit,
		SubredditSection,
	},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum BrowseCommand
{
	Subreddit
	{
		subreddit_name: String,
		#[command(subcommand)]
		section:        SubredditSection,
	},
	Submission
	{
		submission_id: String
	},
	Comment
	{
		comment_id: String
	},
	User
	{
		user_name: String
	},
}

pub enum BrowseResponse
{
	Subreddit(SubredditResponse),
	Submission(SubmissionData),
	Comment(CommentData),
	User(User),
}

#[derive(Debug)]
pub enum SubredditResponse
{
	About(SubredditData),
	Submissions(Submissions),
}

impl std::fmt::Debug for BrowseResponse
{
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result
	{
		match self
		{
			Self::Subreddit(arg0) => f.debug_tuple("Subreddit").field(arg0).finish(),
			Self::Submission(arg0) => f.debug_tuple("Submission").field(arg0).finish(),
			Self::Comment(arg0) => f.debug_tuple("Comment").field(arg0).finish(),
			Self::User(arg0) => f.debug_tuple("User").field(&arg0.user).finish(),
		}
	}
}

pub async fn browse(
	browse: BrowseCommand,
	options: Option<FeedOption>,
) -> BrowseResponse
{
	println!("{:?}", browse);
	match browse
	{
		BrowseCommand::Subreddit {
			subreddit_name,
			section: SubredditSection::About,
		} =>
		{
			BrowseResponse::Subreddit(SubredditResponse::About(
				BrowseSubreddit::about(subreddit_name)
					.await
					.expect("Failed to browse subreddit."),
			))
		}
		BrowseCommand::Subreddit {
			subreddit_name,
			section: SubredditSection::Submissions { sort, limit },
		} =>
		{
			BrowseResponse::Subreddit(SubredditResponse::Submissions(
				BrowseSubreddit::submissions(subreddit_name, sort, limit, options)
					.await
					.expect("Failed to browse subreddit."),
			))
		}
		BrowseCommand::Submission { submission_id: post_id } => BrowseResponse::Submission(browse_submission(post_id)),
		BrowseCommand::Comment { comment_id } => BrowseResponse::Comment(browse_comment(comment_id)),
		BrowseCommand::User { user_name } => BrowseResponse::User(browse_user(user_name)),
	}
}

fn browse_submission(post_id: String) -> SubmissionData
{
	println!("Browsing submission: {}", post_id);
	todo!()
}

fn browse_comment(comment_id: String) -> CommentData
{
	println!("Browsing comment: {}", comment_id);
	todo!()
}
fn browse_user(user_name: String) -> User
{
	println!("Browsing user: {}", user_name);
	todo!()
}
