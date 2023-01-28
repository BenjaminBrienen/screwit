use clap::Subcommand;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Subcommand)]
pub enum SettingCommand
{
	Set
	{
		key: String, value: u32
	},
	Get
	{
		key: String
	},
}

pub fn setting(setting: SettingCommand)
{
	println!("{:?}", setting);
}
