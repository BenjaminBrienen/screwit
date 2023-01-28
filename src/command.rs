use {
	crate::{
		browse::BrowseCommand,
		settings::SettingCommand,
	},
	clap::{
		Parser,
		Subcommand,
	},
};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Subcommand)]
pub enum Command
{
	Test
	{
		value: u32,
	},
	Browse
	{
		#[command(subcommand)]
		object: BrowseCommand,
	},
	Setting
	{
		#[command(subcommand)]
		object: SettingCommand,
	},
	Example,
}

#[derive(Parser, Debug)]
pub struct Cli
{
	#[command(subcommand)]
	pub command: Command,
}
