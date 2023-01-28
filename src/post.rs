use {
	crate::comment::iterate_replies,
	roux::{
		comment::CommentData,
		response::BasicThing,
		Subreddit,
	},
	screwit::*,
};

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
