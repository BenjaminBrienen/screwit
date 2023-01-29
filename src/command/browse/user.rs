use {
	roux::{
		util::{
			FeedOption,
			TimePeriod,
		},
		User,
	},
	std::{
		collections::HashMap,
		default::default,
	},
};

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
