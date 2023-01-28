use {
	crate::user::get_user_participated_subreddits_and_subreddit_karmas,
	roux::User,
};

// #[test_case("000111222333444555666777888999888777666555444333222111000", 1,
// "|000111222333444555666777888999888777666555444333222111000";           "1
// indent level")] #[test_case("
// 000111222333444555666777888999888777666555444333222111000", 2,
// "||000111222333444555666777888999888777666555444333222111000"; "2 indent
// level")] #[test_case("01234567890123456789", 1, "|01234567890123456789"; "two
// tens")] fn test_format_string(
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
