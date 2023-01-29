pub mod browse;
pub mod settings;

use {
	crate::{
		browse::{
			BrowseCommand,
			BrowseResponse,
		},
		settings::{
			SettingCommand,
			SettingResponse,
		},
	},
	clap::{
		Parser,
		Subcommand,
	},
};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Subcommand)]
pub enum Command
{
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
	Example {},
}

#[derive(Debug)]
pub enum Response
{
	Browse(BrowseResponse),
	Setting(SettingResponse),
	Example(String),
}

#[derive(Parser, Debug)]
pub struct Cli
{
	#[command(subcommand)]
	pub command: Command,
}
