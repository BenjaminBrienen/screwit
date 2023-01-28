use clap::Subcommand;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum BrowseCommand
{
	Subreddit
	{
		subreddit_name: String
	},
	Post
	{
		post_id: String
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

pub fn browse(browse: BrowseCommand)
{
	println!("{:?}", browse);
	match browse
	{
		BrowseCommand::Subreddit { subreddit_name } => browse_subreddit(subreddit_name),
		BrowseCommand::Post { post_id } => browse_post(post_id),
		BrowseCommand::Comment { comment_id } => browse_comment(comment_id),
		BrowseCommand::User { user_name } => browse_user(user_name),
	}
}

fn browse_subreddit(subreddit_name: String)
{
	println!("Browsing post: {}", subreddit_name);
	todo!()
}

fn browse_post(post_id: String)
{
	println!("Browsing post: {}", post_id);
	todo!()
}

fn browse_comment(comment_id: String)
{
	println!("Browsing post: {}", comment_id);
	todo!()
}
fn browse_user(user_name: String)
{
	println!("Browsing post: {}", user_name);
	todo!()
}
