use {
	crate::user::{
		filter_subreddits_above_karma_threshhold,
		get_user_participated_subreddits_and_subreddit_karmas,
	},
	async_recursion::async_recursion,
	roux::{
		comment::CommentData,
		response::BasicThing,
		MaybeReplies,
		User,
	},
	screwit::{
		FilterAction,
		Severity,
		SeverityPolicy,
		SubredditPolicies,
	},
	std::default::default,
};

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
			let user_participated_subreddits = get_user_participated_subreddits_and_subreddit_karmas(User::new(username.as_str())).await;
			let filtered_user_participated_subreddits = filter_subreddits_above_karma_threshhold(user_participated_subreddits, 0);
			for subreddit in filtered_user_participated_subreddits
			{
				for subreddit_policy in subreddit_policies
				{
					let is_subreddit_in_policy = subreddit_policy.regex.is_match(subreddit.as_str());
					if is_subreddit_in_policy
					{
						let is_user_worse_than_previously_thought = username_severity < subreddit_policy.severity;
						if is_user_worse_than_previously_thought
						{
							username_severity = subreddit_policy.severity;
						}
						let should_tag_username = severity_policy[username_severity] == FilterAction::Tag;
						if should_tag_username
						{
							tagged_subreddits.push(subreddit);
							break
						}
					}
				}
			}
			let security_policy_for_username = severity_policy[username_severity];
			match security_policy_for_username
			{
				FilterAction::Ignore => rtn.push_str(&username),
				FilterAction::Tag =>
				{
					let all_tagged_subreddits = tagged_subreddits.concat();
					let username_with_tagged_subreddits = &format!("{} {}", username, all_tagged_subreddits);
					rtn.push_str(username_with_tagged_subreddits)
				}
				FilterAction::Filter => (),
				FilterAction::Report => (),
			}
		}
		if let Some(text) = comment.data.body.clone()
		{
			rtn.push_str(&text);
		}
		iterate_all_subreplies(&comment.data.replies, &mut rtn, depth, subreddit_policies, severity_policy).await;
	}
	rtn
}

async fn iterate_all_subreplies(
	replies: &Option<MaybeReplies>,
	rtn: &mut String,
	depth: usize,
	subreddit_policies: &SubredditPolicies,
	severity_policy: &SeverityPolicy,
)
{
	while let Some(MaybeReplies::Reply(reply)) = replies.iter().next()
	{
		let sub_replies = &iterate_replies(&reply.data.children, depth + 1, subreddit_policies, severity_policy).await;
		rtn.push_str(sub_replies);
	}
}
