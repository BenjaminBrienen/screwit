use {
	clap::{
		Subcommand,
		ValueEnum,
	},
	roux::{
		comment::CommentData,
		submission::SubmissionData,
		subreddit::response::SubredditData,
		util::{
			FeedOption,
			RouxError,
		},
		Submissions,
		Subreddit,
		User,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum SubredditSection
{
	About,
	Submissions
	{
		sort:  SortMode,
		limit: u32,
	},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SortMode
{
	New,
	Hot,
	Top,
	Rising,
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
				browse_subreddit_about(subreddit_name)
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
				browse_subreddit_submissions(subreddit_name, sort, limit, options)
					.await
					.expect("Failed to browse subreddit."),
			))
		}
		BrowseCommand::Submission { submission_id: post_id } => BrowseResponse::Submission(browse_submission(post_id)),
		BrowseCommand::Comment { comment_id } => BrowseResponse::Comment(browse_comment(comment_id)),
		BrowseCommand::User { user_name } => BrowseResponse::User(browse_user(user_name)),
	}
}

async fn browse_subreddit_about(subreddit_name: String) -> Result<SubredditData, RouxError>
{
	println!("Browsing subreddit: {}", subreddit_name);
	roux::Subreddit::new(subreddit_name.as_str()).about().await
}

async fn browse_subreddit_submissions(
	subreddit_name: String,
	sort: SortMode,
	limit: u32,
	options: Option<FeedOption>,
) -> Result<Submissions, RouxError>
{
	println!("Browsing subreddit: {}", subreddit_name);
	let subreddit = Subreddit::new(subreddit_name.as_str());
	match sort
	{
		SortMode::New => subreddit.latest(limit, options).await,
		SortMode::Hot => subreddit.hot(limit, options).await,
		SortMode::Top => subreddit.top(limit, options).await,
		SortMode::Rising => subreddit.rising(limit, options).await,
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
