use roux::{
	response::{
		BasicThing,
		Listing,
	},
	submission::SubmissionData,
	util::RouxError,
	Subreddit,
};

pub async fn get_posts() -> Result<BasicThing<Listing<BasicThing<SubmissionData>>>, RouxError>
{
	let subreddit = Subreddit::new("gaming");
	subreddit.hot(1, None).await
}
