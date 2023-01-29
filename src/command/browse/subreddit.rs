use {
	clap::{
		Subcommand,
		ValueEnum,
	},
	roux::{
		self,
		subreddit::response::SubredditData,
		util::{
			FeedOption,
			RouxError,
		},
		Submissions,
		Subreddit,
	},
};

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

pub struct BrowseSubreddit;

impl BrowseSubreddit
{
	pub async fn about(subreddit_name: String) -> Result<SubredditData, RouxError>
	{
		println!("Browsing subreddit: {}", subreddit_name);
		roux::Subreddit::new(subreddit_name.as_str()).about().await
	}

	pub async fn submissions(
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
}
